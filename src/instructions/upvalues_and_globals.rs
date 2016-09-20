use instruction::*;
use upvalues::*;

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
            if let Type::Table(ref upvalue) = *context.ci().upvalue(self.upvalue) {
                let table = upvalue.lock();
                let nil = Type::Nil;
                table.get(&key).unwrap_or(&nil).clone()
            } else {
                panic!("GetTabUp upvalue must be of type Type::Table (got {:?})", context.ci().upvalue(self.upvalue))
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
pub struct SetTabUp {
    pub upval: UpvalueIndex,
    pub key: DataSource,
    pub value: DataSource
}

impl LoadInstruction for SetTabUp {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        SetTabUp {
            upval: a.into(),
            key: b.into(),
            value: c.into(),
        }
    }
}

impl InstructionOps for SetTabUp {
    fn exec(&self, context: &mut Context) {
        let upval = context.ci().upvalue(self.upval).clone();
        let key = self.key.get_from(context);
        let value = self.value.get_from(context);
        let table = as_type_variant!(upval, Type::Table);
        table.lock().insert(key, value);
    }
}