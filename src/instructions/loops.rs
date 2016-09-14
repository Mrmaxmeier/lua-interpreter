use instruction::*;
use types::Number;

// FORLOOP,     A sBx   R(A)+=R(A+2);                                   39
//                        if R(A) <?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForLoop { pub a: Reg, pub jump: isize }

impl LoadInstruction for ForLoop {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_sBx(d);
        ForLoop {
            a: a,
            jump: b,
        }
    }
}

impl InstructionOps for ForLoop {
    fn exec(&self, context: &mut Context) {
        if let (
            Type::Number(Number::Integer(current)),
            Type::Number(Number::Integer(limit)),
            Type::Number(Number::Integer(step))
        ) = (
            context.stack[self.a].as_type(),
            context.stack[self.a + 1].as_type(),
            context.stack[self.a + 2].as_type()
        ) {
            let current = current + step;
            context.stack[self.a] = Type::Number(Number::Integer(current)).into();
            if (step > 0 && current <= limit) || (step < 0 && current >= limit) {
                context.stack[self.a + 3] = context.stack[self.a].as_type().into();
                context.ci_mut().pc += self.jump;
            }
        } else {
            panic!("invalid FORPREP types")
        }
    }
}

// FORPREP,     A sBx   R(A)-=R(A+2); pc+=sBx                           40
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForPrep { pub a: Reg, pub jump: isize }

impl LoadInstruction for ForPrep {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_sBx(d);
        ForPrep {
            a: a,
            jump: b,
        }
    }
}

impl InstructionOps for ForPrep {
    fn exec(&self, context: &mut Context) {
        if let (Type::Number(Number::Integer(val)), Type::Number(Number::Integer(add))) = (context.stack[self.a].as_type(), context.stack[self.a + 2].as_type()) {
            context.stack[self.a] = Type::Number(Number::Integer(val - add)).into();
        } else {
            panic!("invalid FORPREP types")
        }
        context.ci_mut().pc += self.jump;
    }
}

// TFORCALL,    A C     R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));  41
// TFORLOOP,    A sBx  if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx } 42

// SETLIST,     A B C   R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B        43

