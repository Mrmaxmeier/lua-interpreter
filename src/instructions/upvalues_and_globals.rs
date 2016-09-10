use instruction::*;

// 06: GETTABUP   A B C   R(A) := UpValue[B][RK(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetTabUp { pub reg: Reg, pub upvalue: Reg, pub constant: DataSource }

impl LoadInstruction for GetTabUp {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        GetTabUp {
            reg: a,
            upvalue: b,
            constant: c.into(),
        }
    }
}

impl InstructionOps for GetTabUp {
    fn exec(&self, closure: &mut ClosureCtx) {
        let key = if let Type::String(key) = self.constant.get_from(closure) {
            key
        } else {
            panic!("GetTabUp key must be of type Type::String")
        };
        let shared_upvalue = closure.upvalues[self.upvalue].lock();
        let upvalue = if let Type::Table(ref upvalue) = *shared_upvalue {
            upvalue
        } else {
            panic!("GetTabUp upvalue must be of type Type::Table (got {:?})", closure.upvalues[self.upvalue])
        };
        let nil = Type::Nil;
        let value = upvalue.get(&key).unwrap_or(&nil);
        closure.stack[self.reg] = value.clone().into();
    }
    fn debug_info(&self, c: InstructionContext) -> Vec<String> {
        c.filter(vec![
            // TODO: reg as local
            None,
            c.pretty_upval(self.upvalue),
            c.pretty_constant(self.constant),
        ])
    }
}

// 08: SETTABUP A B C   UpValue[A][RK(B)] := RK(C)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SetTabUp { pub reg: Reg, pub upvalue: Reg, pub constant: DataSource }

impl LoadInstruction for SetTabUp {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        SetTabUp {
            reg: a,
            upvalue: b,
            constant: c.into(),
        }
    }
}