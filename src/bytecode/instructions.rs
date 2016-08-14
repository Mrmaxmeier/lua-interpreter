// http://www.lua.org/source/5.3/lopcodes.h.html
use bytecode::parser::*;
use bytecode::interpreter::Interpreter;
use byteorder;

pub trait TInstruction: Sized {
    fn step(&self, &mut Interpreter);
    fn load(u32) -> Self;
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    MOVE(Move),
    LOADK(LoadK),
    GETTABUP(GetTabUp),
    CALL(Call),
    RETURN(Return),
}


impl Parsable for Instruction {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let data = r.read_u32::<byteorder::LittleEndian>().unwrap();
        let opcode = data & 0b00111111;
        println!("opcode: {:?}\tdata: {:#0b}", opcode, data);
        match opcode {
            00 => Instruction::MOVE(Move::load(data)),
            01 => Instruction::LOADK(LoadK::load(data)),
            // TODO: 2 - 5
            06 => Instruction::GETTABUP(GetTabUp::load(data)),
            // TODO: 7 - 37
            36 => Instruction::CALL(Call::load(data)),
            // TODO: 37 TAILCALL
            38 => Instruction::RETURN(Return::load(data)),
            // TODO: ...
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
    let a = d >> 6;
    let b = d >> (6 + 8);
    (a as Reg, b as Reg)
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
// MOVE,/*      A B     R(A) := R(B)                                    */
// LOADK,/*     A Bx    R(A) := Kst(Bx)                                 */
// LOADKX,/*    A       R(A) := Kst(extra arg)                          */
// LOADBOOL,/*  A B C   R(A) := (Bool)B; if (C) pc++                    */
// LOADNIL,/*   A B     R(A), R(A+1), ..., R(A+B) := nil                */
// GETUPVAL,/*  A B     R(A) := UpValue[B]                              */

// GETTABUP,/*  A B C   R(A) := UpValue[B][RK(C)]                       */
// GETTABLE,/*  A B C   R(A) := R(B)[RK(C)]                             */

// SETTABUP,/*  A B C   UpValue[A][RK(B)] := RK(C)                      */
// SETUPVAL,/*  A B     UpValue[B] := R(A)                              */
// SETTABLE,/*  A B C   R(A)[RK(B)] := RK(C)                            */

// NEWTABLE,/*  A B C   R(A) := {} (size = B,C)                         */

// SELF,/*      A B C   R(A+1) := R(B); R(A) := R(B)[RK(C)]             */

// ADD,/*       A B C   R(A) := RK(B) + RK(C)                           */
// SUB,/*       A B C   R(A) := RK(B) - RK(C)                           */
// MUL,/*       A B C   R(A) := RK(B) * RK(C)                           */
// MOD,/*       A B C   R(A) := RK(B) % RK(C)                           */
// POW,/*       A B C   R(A) := RK(B) ^ RK(C)                           */
// DIV,/*       A B C   R(A) := RK(B) / RK(C)                           */
// IDIV,/*      A B C   R(A) := RK(B) // RK(C)                          */
// BAND,/*      A B C   R(A) := RK(B) & RK(C)                           */
// BOR,/*       A B C   R(A) := RK(B) | RK(C)                           */
// BXOR,/*      A B C   R(A) := RK(B) ~ RK(C)                           */
// SHL,/*       A B C   R(A) := RK(B) << RK(C)                          */
// SHR,/*       A B C   R(A) := RK(B) >> RK(C)                          */
// UNM,/*       A B     R(A) := -R(B)                                   */
// BNOT,/*      A B     R(A) := ~R(B)                                   */
// NOT,/*       A B     R(A) := not R(B)                                */
// LEN,/*       A B     R(A) := length of R(B)                          */

// CONCAT,/*    A B C   R(A) := R(B).. ... ..R(C)                       */

// JMP,/*       A sBx   pc+=sBx; if (A) close all upvalues >= R(A - 1)  */
// EQ,/*        A B C   if ((RK(B) == RK(C)) ~= A) then pc++            */
// LT,/*        A B C   if ((RK(B) <  RK(C)) ~= A) then pc++            */
// LE,/*        A B C   if ((RK(B) <= RK(C)) ~= A) then pc++            */

// TEST,/*      A C     if not (R(A) <=> C) then pc++                   */
// TESTSET,/*   A B C   if (R(B) <=> C) then R(A) := R(B) else pc++     */

// CALL,/*      A B C   R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
// TAILCALL,/*  A B C   return R(A)(R(A+1), ... ,R(A+B-1))              */
// RETURN,/*    A B     return R(A), ... ,R(A+B-2)      (see note)      */

// FORLOOP,/*   A sBx   R(A)+=R(A+2);
//                         if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
// FORPREP,/*   A sBx   R(A)-=R(A+2); pc+=sBx                           */

// TFORCALL,/*  A C     R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));  */
// TFORLOOP,/*  A sBx   if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/

// SETLIST,/*   A B C   R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B        */

// CLOSURE,/*   A Bx    R(A) := closure(KPROTO[Bx])                     */

// VARARG,/*    A B     R(A), R(A+1), ..., R(A+B-2) = vararg            */

// EXTRAARG/*   Ax      extra (larger) argument for previous opcode     */

type Reg = isize;

// 0: MOVE   A B   R(A) := R(B)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move { pub to: Reg, pub from: Reg }

impl TInstruction for Move {
    fn step(&self, _: &mut Interpreter) {}
    fn load(d: u32) -> Self {
        let (to, from) = parse_A_B(d);
        Move {
            to: to,
            from: from,
        }
    }
}

// 1: LOADK   A Bx   R(A) := Kst(Bx)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LoadK { pub a: Reg, pub b: Reg }

impl TInstruction for LoadK {
    fn step(&self, _: &mut Interpreter) {}
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_Bx(d);
        LoadK {
            a: a,
            b: b,
        }
    }
}

// 06: GETTABUP   A B C   R(A) := UpValue[B][RK(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GetTabUp { pub a: Reg, pub b: Reg, pub c: Reg }

impl TInstruction for GetTabUp {
    fn step(&self, _: &mut Interpreter) {}
    fn load(d: u32) -> Self {
        let (a, b, c) = parse_A_B_C(d);
        GetTabUp {
            a: a,
            b: b,
            c: c,
        }
    }
}


// 36: CALL     A B C   R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1))
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Call { pub a: Reg, pub b: Reg, pub c: Reg }

impl TInstruction for Call {
    fn step(&self, _: &mut Interpreter) {}
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

impl TInstruction for Return {
    fn step(&self, _: &mut Interpreter) {}
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