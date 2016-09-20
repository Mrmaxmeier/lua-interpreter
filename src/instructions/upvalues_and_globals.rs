use instruction::*;

// 06: GETTABUP   A B C   R(A) := UpValue[B][RK(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetTabUp { pub reg: Reg, pub upvalue: UpvalueIndex, pub constant: DataSource }

impl LoadInstruction for GetTabUp {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        GetTabUp {
            reg: a,
            upvalue: b.into(),
            constant: c.into(),
        }
    }
}

impl InstructionOps for GetTabUp {
    fn exec(&self, context: &mut Context) {
        let key = self.constant.get_from(context);
        let value = {
            if let Type::Table(ref upvalue) = context.ci().upvalues[self.upvalue] {
                let table = upvalue.lock();
                let nil = Type::Nil;
                table.get(&key).unwrap_or(&nil).clone()
            } else {
                panic!("GetTabUp upvalue must be of type Type::Table (got {:?})", context.ci().upvalues[self.upvalue])
            }
        };
        context.stack[self.reg] = value.into();
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