use instruction::Instruction;
use bytecode::Bytecode;
use function_block::FunctionBlock;
use types::{Type, SharedType};
use env::Environment;

#[derive(Debug, Clone, PartialEq)]
pub struct PC {
    _pc: usize,
    _instructions: Vec<Instruction>,
}

impl PC {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        PC {
            _pc: 0,
            _instructions: instructions,
        }
    }

    pub fn at_end(&self) -> bool {
        self._pc >= self._instructions.len()
    }
}

impl PC {
    pub fn get(&self, relative_index: isize) -> &Instruction {
        let index = self._pc as isize + relative_index;
        &self._instructions[index as usize]
    }
    pub fn current(&self) -> &Instruction { self.get(0) }
    pub fn skip(&mut self, n: usize) { *self += n as isize }
}

impl ::std::ops::AddAssign<isize> for PC {
    fn add_assign(&mut self, _rhs: isize) {
        self._pc = (self._pc as isize + _rhs) as usize;
    }
}

#[derive(Debug, Clone)]
pub enum StackEntry {
    Type(Type),
    SharedType(SharedType),
    // TODO: stack barriers / guards?
}

impl StackEntry {
    pub fn as_type(&self) -> Type {
        match *self {
            StackEntry::Type(ref t) => t.clone(),
            StackEntry::SharedType(ref t) => (*t).lock().clone(),
        }
    }
}

impl From<Type> for StackEntry {
    fn from(t: Type) -> Self {
        StackEntry::Type(t)
    }
}

pub type Stack = Vec<StackEntry>;

pub trait StackT {
    fn set_r<T: Into<StackEntry>>(&mut self, usize, T); // TODO: rename set_r
    fn top(&self) -> usize;
}

impl StackT for Stack {
    fn set_r<T: Into<StackEntry>>(&mut self, i: usize, t: T) {
        if self.len() < i + 1 {
            self.push(t.into())
        } else {
            self[i] = t.into()
        }
    }
    fn top(&self) -> usize {
        self.len() - 1
    }
}

pub struct RunResult {
    instruction_count: usize
}

#[derive(Debug, Clone)]
pub struct ClosureCtx {
    pub pc: PC,
    pub stack: Stack,
    pub func: FunctionBlock,
    pub upvalues: Vec<SharedType>,
}

impl ClosureCtx {
    pub fn new(func: FunctionBlock, env: SharedType) -> Self {
        let upvalues = if let Some(upval) = func.upvalues.get(0) {
            assert_eq!(upval.name, Some("_ENV".into()));
            assert_eq!(upval.instack, true);
            vec![env]
        } else {
            vec![]
        };
        ClosureCtx {
            pc: PC::new(func.instructions.clone()),
            stack: vec![],
            upvalues: upvalues,
            func: func
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub cl_stack: Vec<ClosureCtx>,
    pub bytecode: Bytecode,
    pub env: SharedType,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode, env: Environment) -> Self {
        let env = env.make();
        let cl = ClosureCtx::new(bytecode.func.clone(), env.clone());
        Interpreter {
            cl_stack: vec![cl],
            bytecode: bytecode,
            env: env,
        }
    }

    pub fn cl(&self) -> &ClosureCtx {
        &self.cl_stack[self.cl_stack.len() - 1]
    }

    pub fn cl_mut(&mut self) -> &mut ClosureCtx {
        let idx = self.cl_stack.len() - 1;
        &mut self.cl_stack[idx]
    }

    pub fn step(&mut self) {
        let instruction = *self.cl().pc.current();
        // println!("{:?}", instruction);
        self.cl_mut().pc += 1;
        instruction.exec(&mut self.cl_mut());
    }

    pub fn debug(&mut self) {
        println!("step {:?}", self.cl().pc.current());
        self.step();
        println!("stack: {:?}", self.cl().stack);
    }

    pub fn run(&mut self, debug: bool) -> RunResult {
        let mut result = RunResult {
            instruction_count: 0
        };
        while !self.cl().pc.at_end() {
            if debug {
                self.debug();
            } else {
                self.step();
            }
            result.instruction_count += 1
        }
        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use bytecode::Bytecode;
    use parser::Parsable;
    use env::Environment;
    use std::io::Cursor;
    use std::sync::mpsc;

    fn interpreter_from_bytes(data: &[u8]) -> (Interpreter, mpsc::Receiver<String>) {
        let bytecode = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let (tx, rx) = mpsc::channel();
        let interpreter = Interpreter::new(bytecode, Environment::Testing(tx));
        (interpreter, rx)
    }

    #[test]
    fn runs_hello_world() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/hello_world"));
        let result = interpreter.run(true);
        assert_eq!(result.instruction_count, 4);
        assert_eq!(rx.recv().unwrap(), "Hello, World!");
    }

    #[test]
    fn runs_a_bunch_of_constants() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/a_bunch_of_constants"));
        interpreter.run(true);
    }

    #[test]
    fn branches_correctly() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/if_conditions"));
        interpreter.run(true);
        assert_eq!(rx.recv().unwrap(), "true is truthy");
        assert_eq!(rx.recv().unwrap(), "false is falsey");
    }

    #[bench]
    fn step_infinite_loop(b: &mut Bencher) {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/loop"));
        b.iter(|| interpreter.step())
    }
}