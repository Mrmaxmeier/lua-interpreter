mod loading;
mod upvalues_and_globals;
mod table_instructions;
mod arithmetic_and_string;
mod jumps_and_calls;
mod loops;
mod closures;
mod relational_and_logic;

// R(x)   - register
// Kst(x) - constant (in constant table)
// RK(x)  - DataSource [if ISK(x) then Kst(INDEXK(x)) else R(x)]

// /*-------------------------------------------------------------------
// name         args    description
// ---------------------------------------------------------------------*/
pub use self::loading::*;
// MOVE,        A B     R(A) := R(B)                                    00
// LOADK,       A Bx    R(A) := Kst(Bx)                                 01
// LOADKX,      A       R(A) := Kst(extra arg)                          02
// LOADBOOL,    A B C   R(A) := (Bool)B; if (C) pc++                    03
// LOADNIL,     A B     R(A), R(A+1), ..., R(A+B) := nil                04
pub use self::upvalues_and_globals::*;
// GETUPVAL,    A B     R(A) := UpValue[B]                              05

// GETTABUP,    A B C   R(A) := UpValue[B][RK(C)]                       06
// GETTABLE,    A B C   R(A) := R(B)[RK(C)]                             07

// SETTABUP,    A B C   UpValue[A][RK(B)] := RK(C)                      08
// SETUPVAL,    A B     UpValue[B] := R(A)                              09
pub use self::table_instructions::*;
// SETTABLE,    A B C   R(A)[RK(B)] := RK(C)                            10

// NEWTABLE,    A B C   R(A) := {} (size = B,C)                         11

// SELF,        A B C   R(A+1) := R(B); R(A) := R(B)[RK(C)]             12

pub use self::arithmetic_and_string::*;
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

pub use self::jumps_and_calls::*;
// JMP,         A sBx   pc+=sBx; if (A) close all upvalues >= R(A - 1)  30
pub use self::relational_and_logic::*;
// EQ,          A B C   if ((RK(B) == RK(C)) ~= A) then pc++            31
// LT,          A B C   if ((RK(B) <  RK(C)) ~= A) then pc++            32
// LE,          A B C   if ((RK(B) <= RK(C)) ~= A) then pc++            33

// TEST,        A C     if not (R(A) <=> C) then pc++                   34
// TESTSET,     A B C   if (R(B) <=> C) then R(A) := R(B) else pc++     35

// CALL,        A B C   R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) 36
// TAILCALL,    A B C   return R(A)(R(A+1), ... ,R(A+B-1))              37
// RETURN,      A B     return R(A), ... ,R(A+B-2)      (see note)      38

pub use self::loops::*;
// FORLOOP,     A sBx   R(A)+=R(A+2);                                   39
//                        if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
// FORPREP,     A sBx   R(A)-=R(A+2); pc+=sBx                           40

// TFORCALL,    A C     R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));  41
// TFORLOOP,    A sBx  if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx } 42

// SETLIST,     A B C   R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B        43

pub use self::closures::*;
// CLOSURE,     A Bx    R(A) := closure(KPROTO[Bx])                     44

// VARARG,      A B     R(A), R(A+1), ..., R(A+B-2) = vararg            45

// EXTRAARG     Ax      extra (larger) argument for previous opcode     46