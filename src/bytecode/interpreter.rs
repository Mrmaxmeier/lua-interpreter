use bytecode::instructions::Instruction;
use bytecode::bytecode::Bytecode;
use bytecode::function_block::FunctionBlock;
use types::Type;

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

pub type Stack = Vec<Type>;

pub trait StackT {
    fn set_r(&mut self, usize, Type); // TODO: rename set_r
}

impl StackT for Stack {
    fn set_r(&mut self, i: usize, t: Type) {
        if self.len() < i + 1 {
            self.push(t)
        } else {
            self[i] = t
        }
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
    pub upvalues: Vec<Type>,
}

impl ClosureCtx {
    pub fn new(func: FunctionBlock) -> Self {
        ClosureCtx {
            pc: PC::new(func.instructions.clone()),
            stack: Vec::new(),
            upvalues: vec![],
            func: func
        }
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub cl_stack: Vec<ClosureCtx>,
    pub bytecode: Bytecode,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode) -> Self {
        let cl = ClosureCtx::new(bytecode.func.clone());
        Interpreter {
            cl_stack: vec![cl],
            bytecode: bytecode,
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
    use bytecode::bytecode::Bytecode;
    use bytecode::parser::Parsable;
    use std::io::Cursor;

    fn interpreter_from_bytes(data: &[u8]) -> Interpreter {
        let bytecode = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        Interpreter::new(bytecode)
    }

    #[test]
    fn runs_hello_world() {
        let mut interpreter = interpreter_from_bytes(include_bytes!("../../fixtures/hello_world"));
        let result = interpreter.run(true);
        assert_eq!(result.instruction_count, 4);
    }

    #[test]
    fn runs_a_bunch_of_constants() {
        let mut interpreter = interpreter_from_bytes(include_bytes!("../../fixtures/a_bunch_of_constants"));
        interpreter.run(true);
    }

    #[test]
    fn branches_correctly() {
        let mut interpreter = interpreter_from_bytes(include_bytes!("../../fixtures/if_conditions"));
        interpreter.run(true);
    }

    #[bench]
    fn step_infinite_loop(b: &mut Bencher) {
        let mut interpreter = interpreter_from_bytes(include_bytes!("../../fixtures/loop"));
        b.iter(|| interpreter.step())
    }
}