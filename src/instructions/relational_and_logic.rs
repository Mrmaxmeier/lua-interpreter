use instruction::*;

// 31: EQ       A B C   if ((RK(B) == RK(C)) ~= A) then pc++
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Equals { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for Equals {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Equals {
            a: a,
            b: b,
            c: c,
        }
    }
}

// 32: LT       A B C   if ((RK(B) <  RK(C)) ~= A) then pc++
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LessThan { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for LessThan {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        LessThan {
            a: a,
            b: b,
            c: c,
        }
    }
}

// 33: LE       A B C   if ((RK(B) <= RK(C)) ~= A) then pc++
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LessThanOrEquals { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for LessThanOrEquals {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        LessThanOrEquals {
            a: a,
            b: b,
            c: c,
        }
    }
}