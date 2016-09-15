use instruction::*;
use function::*;
// 44: CLOSURE  A Bx    R(A) := closure(KPROTO[Bx])

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Closure { pub a: Reg, pub b: Reg }

impl LoadInstruction for Closure {
    fn load(d: u32) -> Self {
        let (a, b) = parse_A_Bx(d);
        Closure {
            a: a,
            b: b,
        }
    }
}


impl InstructionOps for Closure {
    fn exec(&self, context: &mut Context) {
        let proto = context.ci().func.protos[self.b].clone();
        let func = LuaFunction::new(proto);
        context.stack[self.a] = Type::Function(Function::Lua(func)).into();
    }
}
