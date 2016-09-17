use instruction::*;
use types::LuaTable;

// GETTABLE,    A B C   R(A) := R(B)[RK(C)]                             07
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetTable { pub a: Reg, pub b: Reg, pub c: Reg }
impl LoadInstruction for GetTable {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        GetTable {
            a: a,
            b: b,
            c: c,
        }
    }
}

// SETTABLE,    A B C   R(A)[RK(B)] := RK(C)                            10
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SetTable { pub a: Reg, pub b: Reg, pub c: Reg }
impl LoadInstruction for SetTable {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        SetTable {
            a: a,
            b: b,
            c: c,
        }
    }
}

impl InstructionOps for SetTable {
    fn exec(&self, context: &mut Context) {
        let table = context.stack[self.a];
        let key = context.stack[self.b];
        let value = context.stack[self.c];
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
        let shared: SharedType = Type::Table(table).into();
        context.stack[self.a] = StackEntry::SharedType(shared);
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