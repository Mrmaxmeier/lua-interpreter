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
        self._pc < self._instructions.len()
    }
}

impl PC {
    pub fn current(&self) -> &Instruction { &self[0] }
    pub fn skip(&mut self, n: usize) { *self += n as isize }
}

impl ::std::ops::AddAssign<isize> for PC {
    fn add_assign(&mut self, _rhs: isize) {
        self._pc = (self._pc as isize + _rhs) as usize;
    }
}

impl ::std::ops::Index<isize> for PC {
    type Output = Instruction;
    fn index(&self, _relative_index: isize) -> &Instruction {
        let index = self._pc as isize + _relative_index;
        &self._instructions[index as usize]
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

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub pc: PC,
    pub stack: Stack,
    pub func: FunctionBlock,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode) -> Self {
        Interpreter {
            pc: PC::new(bytecode.func.instructions.clone()),
            stack: Vec::new(),
            func: bytecode.func,
        }
    }

    pub fn step(&mut self) {
        let instruction = *self.pc.current();
        // println!("{:?}", instruction);
        self.pc += 1;
        instruction.exec(self);
    }

    pub fn debug(&mut self) {
        println!("step {:?}", self.pc.current());
        self.step();
        println!("stack: {:?}", self.stack);
    }

    pub fn run(&mut self) {
        while !self.pc.at_end() {
            self.step()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use bytecode::bytecode::Bytecode;
    use bytecode::parser::Parsable;
    use std::io::Cursor;

    #[test]
    fn runs_a_bunch_of_constants() {
        let data = include_bytes!("../../fixtures/a_bunch_of_constants");
        let bytecode = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let mut interpreter = Interpreter::new(bytecode);
        interpreter.run();
    }

    #[test]
    fn branches_correctly() {
        let data = include_bytes!("../../fixtures/if_conditions");
        let bytecode = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let mut interpreter = Interpreter::new(bytecode);
        interpreter.run();
    }

    #[bench]
    fn step_infinite_loop(b: &mut Bencher) {
        let data = include_bytes!("../../fixtures/loop");
        let bytecode = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let mut interpreter = Interpreter::new(bytecode);
        b.iter(|| interpreter.step())
    }
}