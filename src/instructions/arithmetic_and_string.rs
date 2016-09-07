use instruction::*;

// 16: MOD      A B C   R(A) := RK(B) % RK(C)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mod { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for Mod {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Mod {
            a: a,
            b: b,
            c: c,
        }
    }
}

// 28: LEN      A B     R(A) := length of R(B)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Len { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for Len {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Len {
            a: a,
            b: b,
            c: c,
        }
    }
}

// 29: CONCAT   A B C   R(A) := R(B).. ... ..R(C)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Concat { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for Concat {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Concat {
            a: a,
            b: b,
            c: c,
        }
    }
}