// http://www.lua.org/source/5.3/lopcodes.h.html
use bytecode::parser::*;
use bytecode::interpreter::Interpreter;
use byteorder;

pub trait LoadInstruction: Sized {
    fn load(u32) -> Self;
}
pub trait InstructionOps: {
    fn exec(&self, &mut Interpreter);
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    MOVE(Move),
    LOADK(LoadK),
    LOADBOOL(LoadBool),
    LOADNIL(LoadNil),
    GETTABUP(GetTabUp),
    JMP(Jmp),
    CALL(Call),
    RETURN(Return),
}

macro_rules! match_trait_as_impl {
    ( $this:expr, [$( $x:path ),*] => as $cast_to:ty ) => {
        match $this {
            $( &$x(ref v) => v as $cast_to, )*
            v => panic!("`as {:?}` not implemented for {:?}", stringify!($cast_to), v) 
        }
    };
}

impl Instruction {
    pub fn as_ops(&self) -> &InstructionOps {
        match_trait_as_impl!(self, [
            Instruction::JMP
        ] => as &InstructionOps)
    }
    pub fn exec(&self, i: &mut Interpreter) {
        self.as_ops().exec(i);
    }
}


impl Parsable for Instruction {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let data = r.read_u32::<byteorder::LittleEndian>().unwrap();
        let opcode = data & 0b00111111;
        println!("opcode: {:?}\tdata: {:#0b}", opcode, data);
        match opcode {
            00 => Instruction::MOVE(Move::load(data)),
            01 => Instruction::LOADK(LoadK::load(data)),
            // TODO: 2 LOADKX
            03 => Instruction::LOADBOOL(LoadBool::load(data)),
            04 => Instruction::LOADNIL(LoadNil::load(data)),
            // TODO: 5 GETUPVAL
            06 => Instruction::GETTABUP(GetTabUp::load(data)),
            // TODO: 7 - 29
            30 => Instruction::JMP(Jmp::load(data)),
            // TODO: 31 - 36
            36 => Instruction::CALL(Call::load(data)),
            // TODO: 37 TAILCALL
            38 => Instruction::RETURN(Return::load(data)),
            // TODO: 39 - 46
            invalid => panic!("invalid opcode: {:?}, instruction: {:?}", invalid, data)
        }
    }
}


#[allow(non_snake_case)]
fn parse_A_B(d: u32) -> (Reg, Reg) {
    let a = (d >> 6) & 0xFF;
    let b = (d >> (6 + 8)) & 0xFF;
    (a as Reg, b as Reg)
}

#[allow(non_snake_case)]
fn parse_A_Bx(d: u32) -> (Reg, Reg) {
    let a = (d >> 6) & 0xFF;
    let b = d >> (6 + 8);
    (a as Reg, b as Reg)
}

// Field sBx can represent negative numbers, but it doesn’t use 2s complement.
// Instead, it has a bias equal to half the maximum integer that can be represented by its unsigned counterpart, Bx.
// For a field size of 18 bits, Bx can hold a maximum unsigned integer value of 262143, and so the bias is 131071 (calculated as 262143 >> 1).
// A value of -1 will be encoded as (-1 + 131071) or 131070 or 1FFFE in hexadecimal.
#[allow(non_snake_case)]
fn parse_A_sBx(d: u32) -> (Reg, isize) {
    let a = (d >> 6) & 0xFF;
    let b = d >> (6 + 8);

    let b_bits = 32 - (6 + 8);
    let b_bias = (1 << (b_bits - 1)) - 1;
    let sBx = b as isize - b_bias;
    (a as Reg, sBx)
}

#[allow(non_snake_case)]
fn parse_A_B_C(d: u32) -> (Reg, Reg, Reg) {
    let a = (d >> 6) & 0xFF;
    let b = (d >> (6 + 8)) & 0xFF;
    let c = (d >> (6 + 8 * 2)) & 0xFF;
    (a as Reg, b as Reg, c as Reg)
}


// /*-------------------------------------------------------------------
// name         args    description
// ---------------------------------------------------------------------*/
// MOVE,        A B     R(A) := R(B)                                    00
// LOADK,       A Bx    R(A) := Kst(Bx)                                 01
// LOADKX,      A       R(A) := Kst(extra arg)                          02
// LOADBOOL,    A B C   R(A) := (Bool)B; if (C) pc++                    03
// LOADNIL,     A B     R(A), R(A+1), ..., R(A+B) := nil                04
// GETUPVAL,    A B     R(A) := UpValue[B]                              05

// GETTABUP,    A B C   R(A) := UpValue[B][RK(C)]                       06
// GETTABLE,    A B C   R(A) := R(B)[RK(C)]                             07

// SETTABUP,    A B C   UpValue[A][RK(B)] := RK(C)                      08
// SETUPVAL,    A B     UpValue[B] := R(A)                              09
// SETTABLE,    A B C   R(A)[RK(B)] := RK(C)                            10

// NEWTABLE,    A B C   R(A) := {} (size = B,C)                         11

// SELF,        A B C   R(A+1) := R(B); R(A) := R(B)[RK(C)]             12

// ADD,         A B C   R(A) := RK(B) + RK(C)                           13
// SUB,         A B C   R(A) := RK(B) - RK(C)                           14
// MUL,         A B C   R(A) := RK(B) * RK(C)                           15
// MOD,         A B C   R(A) := RK(B) % RK(C)                           16
// POW,         A B C   R(A) := RK(B) ^ RK(C)                           17
// DIV,         A B C   R(A) := RK(B) / RK(C)                           18
// IDIV,        A B C   R(A) := RK(B) // RK(C)                          19
// BAND,        A B C   R(A) := RK(B) & RK(C)                           20
// BOR,         A B C   R(A) := RK(B) | RK(C)                           21
// BXOR,        A B C   R(A) := RK(B) ~ RK(C)                           22
// SHL,         A B C   R(A) := RK(B) << RK(C)                          23
// SHR,         A B C   R(A) := RK(B) >> RK(C)                          24
// UNM,         A B     R(A) := -R(B)                                   25
// BNOT,        A B     R(A) := ~R(B)                                   26
// NOT,         A B     R(A) := not R(B)                                27
// LEN,         A B     R(A) := length of R(B)                          28

// CONCAT,      A B C   R(A) := R(B).. ... ..R(C)                       29

// JMP,         A sBx   pc+=sBx; if (A) close all upvalues >= R(A - 1)  30
// EQ,          A B C   if ((RK(B) == RK(C)) ~= A) then pc++            31
// LT,          A B C   if ((RK(B) <  RK(C)) ~= A) then pc++            32
// LE,          A B C   if ((RK(B) <= RK(C)) ~= A) then pc++            33

// TEST,        A C     if not (R(A) <=> C) then pc++                   34
// TESTSET,     A B C   if (R(B) <=> C) then R(A) := R(B) else pc++     35

// CALL,        A B C   R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) 36
// TAILCALL,    A B C   return R(A)(R(A+1), ... ,R(A+B-1))              37
// RETURN,      A B     return R(A), ... ,R(A+B-2)      (see note)      38

// FORLOOP,     A sBx   R(A)+=R(A+2);                                   39
//                        if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
// FORPREP,     A sBx   R(A)-=R(A+2); pc+=sBx                           40

// TFORCALL,    A C     R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));  41
// TFORLOOP,    A sBx  if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx } 42

// SETLIST,     A B C   R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B        43

// CLOSURE,     A Bx    R(A) := closure(KPROTO[Bx])                     44

// VARARG,      A B     R(A), R(A+1), ..., R(A+B-2) = vararg            45

// EXTRAARG     Ax      extra (larger) argument for previous opcode     46

type Reg = isize;

// 00: MOVE   A B   R(A) := R(B)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move { pub to: Reg, pub from: Reg }

impl LoadInstruction for Move {
    fn load(d: u32) -> Self {
        let (to, from) = parse_A_B(d);
        Move {
            to: to,
            from: from,
        }
    }
}

// 01: LOADK   A Bx   R(A) := Kst(Bx)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadK { pub local: Reg, pub constant: Reg }

impl LoadInstruction for LoadK {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_Bx(d);
        LoadK {
            local: a,
            constant: b,
        }
    }
}


// 03: LOADBOOL     A B C       R(A) := (Bool)B; if (C) pc++
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadBool { pub reg: Reg, pub value: bool, pub jump: bool }

impl LoadInstruction for LoadBool {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        LoadBool {
            reg: a,
            value: b > 0,
            jump: c > 0,
        }
    }
}


// 04: LOADNIL      A B         R(A), R(A+1), ..., R(A+B) := nil
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadNil { pub a: Reg, pub b: Reg }

impl LoadInstruction for LoadNil {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_Bx(d);
        LoadNil {
            a: a,
            b: b,
        }
    }
}

// 06: GETTABUP   A B C   R(A) := UpValue[B][RK(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetTabUp { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for GetTabUp {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        GetTabUp {
            a: a,
            b: b,
            c: c,
        }
    }
}

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
    fn exec(&self, interpreter: &mut Interpreter) {
        interpreter.pc += self.jump;
    }
}


// 36: CALL     A B C   R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Call { pub a: Reg, pub b: Reg, pub c: Reg }

impl LoadInstruction for Call {
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        Call {
            a: a,
            b: b,
            c: c,
        }
    }
}

// 38: RETURN   A B     return R(A), ... ,R(A+B-2)
// if (B == 0) then return up to 'top'.
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



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor; 
    use bytecode::parser::Parsable;

    #[test]
    fn parses_return_instruction() {
        let data = &[0x26, 0x00, 0x80, 0x00];
        let mut reader = Cursor::new(data);
        let instruction = Instruction::parse(&mut reader);
        assert_eq!(instruction, Instruction::RETURN(Return {a: 0, b: 0}));
    }
}