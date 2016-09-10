use instruction::*;
use function;
use function::{Function, NativeFunction};
use std::sync::Arc;
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
    fn exec(&self, closure: &mut ClosureCtx) {
        closure.pc += self.jump;
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
pub struct Test { pub value: Reg, constant: Reg }

impl LoadInstruction for Test {
    fn load(d: u32) -> Self {
        let (a, _, c) = parse_A_B_C(d);
        Test {
            value: a,
            constant: c
        }
    }
}

impl InstructionOps for Test {
    fn exec(&self, closure: &mut ClosureCtx) {
        let val = &closure.stack[self.value].as_type();
        let constant = &closure.func.constants[self.constant];
        if val == constant {
            closure.pc.skip(1);
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
    fn call_native(&self, closure: &mut ClosureCtx, native: Arc<Mutex<NativeFunction>>) {
        let return_slots = match self.returns {
            Count::Unknown => self.function..closure.stack.top(),
            Count::Known(count) => self.function..self.function + count,
        };

        let call_returns = {
            let param_start = self.function + 1;
            let params = match self.params {
                Count::Unknown => &closure.stack[param_start..closure.stack.top()],
                Count::Known(count) => &closure.stack[param_start..param_start + count],
            };
            let mut call_info = function::FunctionInterface::new(params);
            native.lock()(&mut call_info);
            call_info.ret
        };

        for (item, index) in call_returns.iter().zip(return_slots) {
            closure.stack[index] = StackEntry::Type(item.clone());
        }
    }
}

impl InstructionOps for Call {
    fn exec(&self, closure: &mut ClosureCtx) {
        println!("Call::exec function: {:?}", closure.stack[self.function]);
        let param_start = self.function + 1;
        {
            let params = match self.params {
                Count::Unknown => &closure.stack[param_start..closure.stack.top()],
                Count::Known(count) => &closure.stack[param_start..param_start + count],
            };
            println!("calling with params: {:?}", params);
        }
        if let StackEntry::Type(Type::Function(func)) = closure.stack[self.function].clone() {
            match func {
                Function::Native(native) => self.call_native(closure, native),
                Function::Lua(_) => unimplemented!(),
            }
        } else {
            panic!("Call function must be of type Type::Function (got {:?})", closure.stack[self.function])
        }
    }
}


// 37 TAILCALL  A B C   return R(A)(R(A+1), ... ,R(A+B-1))
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tailcall {
    pub a: Reg,
    pub b: Reg,
    pub c: Reg,
}

impl LoadInstruction for Tailcall {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Tailcall {
            a: a,
            b: b,
            c: c,
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
pub struct Return { pub a: Reg, pub b: Reg }

impl LoadInstruction for Return {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_B(d);
        Return {
            a: a,
            b: b,
        }
    }
}

impl InstructionOps for Return {
    fn exec(&self, _: &mut ClosureCtx) {
        // TODO: implement Return.exec
    }

    fn debug_info(&self, _: InstructionContext) -> Vec<String> {
        if self.b == 0 {
            vec!["return to top".to_owned()]
        } else if self.b == 1 {
            vec!["no return values".to_owned()]
        } else {
            unimplemented!()
        }
    }
}