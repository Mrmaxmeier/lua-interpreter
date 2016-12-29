use instruction::*;


// 05: GETUPVAL   A B     R(A) := UpValue[B]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetUpval { pub reg: Reg, pub upvalue: usize }

impl LoadInstruction for GetUpval {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_B(d);
        GetUpval {
            reg: a,
            upvalue: b,
        }
    }
}

impl InstructionOps for GetUpval {
    fn exec(&self, context: &mut Context) {
        let upval = context.ci().upvalues[self.upvalue].clone();
        context.stack[self.reg] = upval.value(context).into();
    }
}

// 06: GETTABUP   A B C   R(A) := UpValue[B][RK(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetTabUp { pub reg: Reg, pub upvalue: usize, pub constant: DataSource }

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
    fn exec(&self, context: &mut Context) {
        let key = self.constant.get_from(context);
        let value = {
            let table = context.ci().upvalues[self.upvalue].value(context);
            if let Type::Table(ref upvalue) = table {
                let table = upvalue.lock();
                let nil = Type::Nil;
                table.get(&key).unwrap_or(&nil).clone()
            } else {
                panic!("GetTabUp upvalue must be of type Type::Table (got {:?})", table)
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
    pub upval: usize,
    pub key: DataSource,
    pub value: DataSource
}

impl LoadInstruction for SetTabUp {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        SetTabUp {
            upval: a,
            key: b.into(),
            value: c.into(),
        }
    }
}

impl InstructionOps for SetTabUp {
    fn exec(&self, context: &mut Context) {
        let upval = context.ci().upvalues[self.upval].clone();
        let key = self.key.get_from(context);
        let value = self.value.get_from(context);
        let table = as_type_variant!(upval.value(&context), Type::Table);
        table.lock().insert(key, value);
    }
}