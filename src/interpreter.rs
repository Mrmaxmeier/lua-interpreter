use instruction::Instruction;
use bytecode::Bytecode;
use function_block::FunctionBlock;
use types::{SharedType};
use env::Environment;
use stack::Stack;

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

pub struct RunResult {
    instruction_count: usize
}

#[derive(Debug, Clone)]
pub struct CallInfo {
    pub pc: PC,
    pub func: FunctionBlock,
    pub upvalues: Vec<SharedType>,
}

impl CallInfo {
    pub fn new(func: FunctionBlock, env: SharedType) -> Self {
        let upvalues = if let Some(upval) = func.upvalues.get(0) {
            assert_eq!(upval.name, Some("_ENV".into()));
            assert_eq!(upval.instack, true);
            vec![env]
        } else {
            vec![]
        };
        CallInfo {
            pc: PC::new(func.instructions.clone()),
            upvalues: upvalues,
            func: func
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Context {
    pub call_info: Vec<CallInfo>,
    pub stack: Stack,
}

impl Context {
    pub fn new() -> Self { Self::default() }
    
    pub fn ci(&self) -> &CallInfo {
        &self.call_info[self.call_info.len() - 1]
    }

    pub fn ci_mut(&mut self) -> &mut CallInfo {
        let index = self.call_info.len() - 1;
        &mut self.call_info[index]
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub context: Context,
    pub bytecode: Bytecode,
    pub env: SharedType,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode, env: Environment) -> Self {
        let env = env.make();
        let mut context = Context::default();
        context.call_info.push(CallInfo::new(bytecode.func.clone(), env.clone()));
        Interpreter {
            context: context,
            bytecode: bytecode,
            env: env,
        }
    }

    pub fn pc(&self) -> &PC {
        &self.context.ci().pc
    }

    pub fn step(&mut self) {
        let instruction = *self.pc().current();
        // println!("{:?}", instruction);
        self.context.ci_mut().pc += 1;
        instruction.exec(&mut self.context);
    }

    pub fn debug(&mut self) {
        println!("pc: {}; {:?}", self.pc()._pc, self.pc().current());
        self.step();
        println!("stack: {}", self.context.stack.repr());
    }

    pub fn run(&mut self) -> RunResult {
        let mut result = RunResult {
            instruction_count: 0
        };
        while !self.pc().at_end() {
            self.step();
            result.instruction_count += 1
        }
        result
    }

    pub fn run_debug(&mut self) -> RunResult {
        let mut result = RunResult {
            instruction_count: 0
        };
        while !self.pc().at_end() {
            self.debug();
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
        let result = interpreter.run_debug();
        assert_eq!(result.instruction_count, 4);
        assert_eq!(rx.recv().unwrap(), "Hello, World!");
    }

    #[test]
    fn runs_a_bunch_of_constants() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/a_bunch_of_constants"));
        interpreter.run_debug();
    }

    #[should_panic]
    #[test]
    fn assert_false_panics() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/assert_false"));
        interpreter.run_debug();
    }

    #[test]
    fn runs_assertions() {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/assertions"));
        interpreter.run_debug();
    }

    #[test]
    fn calls_lua_functions() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/function"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "");
    }

    #[test]
    fn branches_correctly() {
        let (mut interpreter, rx) = interpreter_from_bytes(include_bytes!("../fixtures/if_conditions"));
        interpreter.run_debug();
        assert_eq!(rx.recv().unwrap(), "true is truthy");
        assert_eq!(rx.recv().unwrap(), "false is falsey");
    }

    #[bench]
    fn step_infinite_loop(b: &mut Bencher) {
        let (mut interpreter, _) = interpreter_from_bytes(include_bytes!("../fixtures/loop"));
        b.iter(|| interpreter.step())
    }
}