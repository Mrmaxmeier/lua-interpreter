// http://www.lua.org/source/5.3/lopcodes.h.html
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    /*-------------------------------------------------------------------
    name         args    description
    ---------------------------------------------------------------------*/
    MOVE,/*      A B     R(A) := R(B)                                    */
    LOADK,/*     A Bx    R(A) := Kst(Bx)                                 */
    LOADKX,/*    A       R(A) := Kst(extra arg)                          */
    LOADBOOL,/*  A B C   R(A) := (Bool)B; if (C) pc++                    */
    LOADNIL,/*   A B     R(A), R(A+1), ..., R(A+B) := nil                */
    GETUPVAL,/*  A B     R(A) := UpValue[B]                              */

    GETTABUP,/*  A B C   R(A) := UpValue[B][RK(C)]                       */
    GETTABLE,/*  A B C   R(A) := R(B)[RK(C)]                             */

    SETTABUP,/*  A B C   UpValue[A][RK(B)] := RK(C)                      */
    SETUPVAL,/*  A B     UpValue[B] := R(A)                              */
    SETTABLE,/*  A B C   R(A)[RK(B)] := RK(C)                            */

    NEWTABLE,/*  A B C   R(A) := {} (size = B,C)                         */

    SELF,/*      A B C   R(A+1) := R(B); R(A) := R(B)[RK(C)]             */

    ADD,/*       A B C   R(A) := RK(B) + RK(C)                           */
    SUB,/*       A B C   R(A) := RK(B) - RK(C)                           */
    MUL,/*       A B C   R(A) := RK(B) * RK(C)                           */
    MOD,/*       A B C   R(A) := RK(B) % RK(C)                           */
    POW,/*       A B C   R(A) := RK(B) ^ RK(C)                           */
    DIV,/*       A B C   R(A) := RK(B) / RK(C)                           */
    IDIV,/*      A B C   R(A) := RK(B) // RK(C)                          */
    BAND,/*      A B C   R(A) := RK(B) & RK(C)                           */
    BOR,/*       A B C   R(A) := RK(B) | RK(C)                           */
    BXOR,/*      A B C   R(A) := RK(B) ~ RK(C)                           */
    SHL,/*       A B C   R(A) := RK(B) << RK(C)                          */
    SHR,/*       A B C   R(A) := RK(B) >> RK(C)                          */
    UNM,/*       A B     R(A) := -R(B)                                   */
    BNOT,/*      A B     R(A) := ~R(B)                                   */
    NOT,/*       A B     R(A) := not R(B)                                */
    LEN,/*       A B     R(A) := length of R(B)                          */

    CONCAT,/*    A B C   R(A) := R(B).. ... ..R(C)                       */

    JMP,/*       A sBx   pc+=sBx; if (A) close all upvalues >= R(A - 1)  */
    EQ,/*        A B C   if ((RK(B) == RK(C)) ~= A) then pc++            */
    LT,/*        A B C   if ((RK(B) <  RK(C)) ~= A) then pc++            */
    LE,/*        A B C   if ((RK(B) <= RK(C)) ~= A) then pc++            */

    TEST,/*      A C     if not (R(A) <=> C) then pc++                   */
    TESTSET,/*   A B C   if (R(B) <=> C) then R(A) := R(B) else pc++     */

    CALL,/*      A B C   R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
    TAILCALL,/*  A B C   return R(A)(R(A+1), ... ,R(A+B-1))              */
    RETURN,/*    A B     return R(A), ... ,R(A+B-2)      (see note)      */

    FORLOOP,/*   A sBx   R(A)+=R(A+2);
                            if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
    FORPREP,/*   A sBx   R(A)-=R(A+2); pc+=sBx                           */

    TFORCALL,/*  A C     R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));  */
    TFORLOOP,/*  A sBx   if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/

    SETLIST,/*   A B C   R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B        */

    CLOSURE,/*   A Bx    R(A) := closure(KPROTO[Bx])                     */

    VARARG,/*    A B     R(A), R(A+1), ..., R(A+B-2) = vararg            */

    EXTRAARG/*   Ax      extra (larger) argument for previous opcode     */
}



named!(pub parse_instruction< Box<Instruction> >, chain!(
    take!(4),
    || { Box::new(Instruction::RETURN) /* TODO */ }
));