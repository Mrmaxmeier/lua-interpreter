use instruction::*;
use table::LuaTable;

// GETTABLE,    A B C   R(A) := R(B)[RK(C)]                             07
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetTable { pub a: Reg, pub b: Reg, pub c: DataSource }
impl LoadInstruction for GetTable {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        GetTable {
            a: a,
            b: b,
            c: c.into(),
        }
    }
}

impl InstructionOps for GetTable {
    fn exec(&self, context: &mut Context) {
        let key = context.stack[self.b].as_type();
        let _table = context.stack[self.b].as_type();
        let table = as_type_variant!(_table, Type::Table);
        let _guard = table.lock();
        let nil = Type::Nil;
        let value = _guard.get(&key).unwrap_or(&nil);
        context.stack[self.a] = value.clone().into();
    }
}

// SETTABLE,    A B C   R(A)[RK(B)] := RK(C)                            10
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SetTable { pub a: Reg, pub b: DataSource, pub c: DataSource }
impl LoadInstruction for SetTable {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        SetTable {
            a: a,
            b: b.into(),
            c: c.into(),
        }
    }
}

impl InstructionOps for SetTable {
    fn exec(&self, context: &mut Context) {
        let key = self.b.get_from(context);
        let value = self.c.get_from(context);
        let _table = context.stack[self.a].as_type();
        let table = as_type_variant!(_table, Type::Table);
        let mut _guard = table.lock();
        _guard.insert(key, value);
    }
}

// NEWTABLE,    A B C   R(A) := {} (size = B,C)                         11
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NewTable { pub a: Reg }
impl LoadInstruction for NewTable {
    fn load(d: u32) -> Self {
        let (a, _, _) = parse_A_B_C(d);
        NewTable {
            a: a,
            // TODO: actually use b and c
        }
    }
}

impl InstructionOps for NewTable {
    fn exec(&self, context: &mut Context) {
        let table = LuaTable::new();
        let as_type = Type::Table(table);
        context.stack[self.a] = as_type.into();
    }
}

// SELF,        A B C   R(A+1) := R(B); R(A) := R(B)[RK(C)]             12
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SelfOp { pub a: Reg, pub b: Reg, pub c: Reg }
impl LoadInstruction for SelfOp {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        SelfOp {
            a: a,
            b: b,
            c: c,
        }
    }
}