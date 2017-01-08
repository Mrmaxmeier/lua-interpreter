use instruction::*;
use function;
use function::{Function, NativeFunction};
use interpreter::{CallInfo, PC};
use std::sync::Arc;
use std::mem;
use parking_lot::Mutex;

// 30: JMP      A sBx   pc += sBx; if (A) close all upvalues >= R(A - 1)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Jmp { pub a: Reg, pub jump: isize }

impl LoadInstruction for Jmp {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_sBx(d);
        Jmp {
            a: a,
            jump: b,
        }
    }
}

impl InstructionOps for Jmp {
    fn exec(&self, context: &mut Context) {
        context.ci_mut().pc += self.jump;
    }

    fn debug_info(&self, c: InstructionContext) -> Vec<String> {
        vec![
            format!("to [{}]", 1 + c.index as isize + self.jump + 1)
        ]
    }
}


// 34: TEST     A C     if not (R(A) <=> C) then pc+
// For the fall-through case, a JMP is always expected, in order to optimize execution in the virtual machine.
// In effect, TEST and TESTSET must always be paired with a following JMP instruction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Test { pub value: Reg, constant: bool }

impl LoadInstruction for Test {
    fn load(d: u32) -> Self {
        let (a, _, c) = parse_A_B_C(d);
        Test {
            value: a,
            constant: c > 0
        }
    }
}

impl InstructionOps for Test {
    fn exec(&self, context: &mut Context) {
        let jump = {
            let val = &context.stack[self.value].as_type();
            val.truethy() != self.constant
        };
        if jump {
            context.ci_mut().pc += 1;
        }
    }
}

// 35: TESTSET  A B C   if (R(B) <=> C) then R(A) := R(B) else pc++
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestSet { pub reg: Reg, pub value: Reg, pub constant: bool }

impl LoadInstruction for TestSet {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        TestSet {
            reg: a,
            value: b,
            constant: c > 0,
        }
    }
}

impl InstructionOps for TestSet {
    fn exec(&self, context: &mut Context) {
        let eq = {
            let value = &context.stack[self.value].as_type();
            value.truethy() != self.constant
        };
        if eq {
            context.stack[self.reg] = context.stack[self.value].as_type().into();
        } else {
            context.ci_mut().pc += 1;
        }
    }
}

// 36: CALL     A B C   R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Call {
    pub function: Reg,
    pub params: Count,
    pub returns: Count,
}

impl LoadInstruction for Call {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Call {
            function: a,
            params: b.into(),
            returns: c.into(),
        }
    }
}

impl Call {
    fn call_native(&self, context: &mut Context, native: Arc<Mutex<NativeFunction>>) {
        let return_slots = match self.returns {
            Count::Unknown => self.function..context.stack.top(),
            Count::Known(count) => self.function..self.function + count,
        };

        let call_returns = {
            let param_start = self.function + 1;
            let params = match self.params {
                Count::Unknown => &context.stack[param_start..context.stack.top()],
                Count::Known(count) => &context.stack[param_start..param_start + count],
            };
            let mut call_info = function::FunctionInterface::new(params);
            native.lock()(&mut call_info);
            call_info.ret
        };

        for (item, index) in call_returns.iter().zip(return_slots) {
            context.stack[index] = StackEntry::Type(item.clone());
        }
    }

    fn call_lua(&self, context: &mut Context, lua: function::LuaFunction) {
        context.ci_mut().pc += -1isize; // re-run this instruction once call has finished
        let encup = context.ci().upvalues.clone();
        let call_info = CallInfo::new(context, lua.proto.clone(), &encup);
        context.call_info.push(call_info);
        let param_start = self.function + 1;
        let param_range = match self.params {
            Count::Unknown => param_start..context.stack.top(),
            Count::Known(count) => param_start..param_start + count,
        };
        let mut params = param_range.map(|i| context.stack[i].as_type()).collect::<Vec<_>>();
        while params.len() < lua.proto.amount_parameters as usize {
            params.push(Type::Nil);
        }
        context.stack.insert_barrier();
        for (i, param) in params.iter().enumerate() {
            context.stack[i] = param.clone().into()
        }
    }

    fn finish_lua_call(&self, context: &mut Context, returns: Vec<Type>) {
        let start = self.function;
        let range = match self.returns {
            Count::Unknown => start..context.stack.top(),
            Count::Known(count) => start..start + count,
        };
        for (i, ret) in range.zip(returns) {
            context.stack[i] = ret.into();
        } 
    }
}

impl InstructionOps for Call {
    fn exec(&self, context: &mut Context) {
        if let Some(returns) = mem::replace(&mut context.ci_mut()._subcall_returns, None) {
            self.finish_lua_call(context, returns);
            return
        }
        if let StackEntry::Type(Type::Function(func)) = context.stack[self.function].clone() {
            match func {
                Function::Native(func) => self.call_native(context, func),
                Function::Lua(func) => self.call_lua(context, func),
            }
        } else {
            panic!("Call function must be of type Type::Function (got {:?})", context.stack[self.function])
        }
    }
}


// 37 TAILCALL  A B C   return R(A)(R(A+1), ... ,R(A+B-1))
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tailcall {
    pub function: Reg,
    pub params: Count,
    pub c: Reg,
}

impl LoadInstruction for Tailcall {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Tailcall {
            function: a,
            params: b.into(),
            c: c,
        }
    }
}

impl InstructionOps for Tailcall {
    fn exec(&self, context: &mut Context) {
        let param_start = self.function + 1;
        let param_range = match self.params {
            Count::Unknown => param_start..context.stack.top(),
            Count::Known(count) => param_start..param_start + count,
        };
        let mut params = param_range.map(|i| context.stack[i].as_type()).collect::<Vec<_>>();

        if let StackEntry::Type(Type::Function(Function::Lua(func))) = context.stack[self.function].clone() {
            while params.len() < func.proto.amount_parameters as usize {
                params.push(Type::Nil);
            }
            let mut ci = context.ci_mut();
            ci.pc = PC::new(func.proto.instructions.clone());
            ci.func = func.proto;
        } else {
            panic!("Tailcall function must be of type Function::Lua (got {:?})", context.stack[self.function])
        }
        let call_base = context.stack.get_level(0);
        context.close_upvalues(call_base);
        context.stack.pop_barrier();
        context.stack.insert_barrier();
        for (i, param) in params.iter().enumerate() {
            context.stack[i] = param.clone().into()
        }
    }
}

// 38: RETURN   A B     return R(A), ... ,R(A+B-2)
// if (B == 0) then return up to 'top'.
//
// Returns to the calling function, with optional return values.
// If B is 1, there are no return values. If B is 2 or more, there are (B-1) return values, located in consecutive registers from R(A) onwards.
// If B is 0, the set of values from R(A) to the top of the stack is returned.
// This form is used when the last expression in the return list is a function call, so the number of actual values returned is indeterminate.
// RETURN also closes any open upvalues, equivalent to a CLOSE instruction.
// See the CLOSE instruction for more information.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Return { pub base: Reg, pub count: Count }

impl LoadInstruction for Return {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_B(d);
        Return {
            base: a,
            count: b.into(),
        }
    }
}

impl InstructionOps for Return {
    fn exec(&self, context: &mut Context) {
        if context.call_info.pop().is_some() {
            let return_range = match self.count {
                Count::Unknown => {
                    unimplemented!()
                },
                Count::Known(count) => {
                    self.base..self.base + count
                }
            };
            let returns: Vec<_> = return_range.map(|index| context.stack[index].as_type()).collect();
            if !context.call_info.is_empty() {
                let call_base = context.stack.get_level(0);
                context.close_upvalues(call_base);
            }
            context.stack.pop_barrier();
            if !context.call_info.is_empty() {
                context.ci_mut()._subcall_returns = Some(returns)
            }
        }
    }

    fn debug_info(&self, _: InstructionContext) -> Vec<String> {
        if self.count == Count::Unknown {
            vec!["return to top".to_owned()]
        } else if self.count == Count::Known(0) {
            vec!["no return values".to_owned()]
        } else {
            vec![format!("return {:?}", self.count)]
        }
    }
}