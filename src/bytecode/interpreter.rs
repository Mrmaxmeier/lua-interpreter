use bytecode::instructions::Instruction;
use bytecode::bytecode::Bytecode;
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
}

impl PC {
    pub fn current(&self) -> &Instruction { &self[0] }
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


#[derive(Debug, Clone)]
pub struct Interpreter {
    pub pc: PC,
    pub stack: Vec<Type>,
}

impl Interpreter {
    pub fn new(bytecode: Bytecode) -> Self {
        Interpreter {
            pc: PC::new(bytecode.func.instructions),
            stack: Vec::new(),
        }
    }
    pub fn step(&mut self) {
        let instruction = *self.pc.current();
        // println!("{:?}", instruction);
        self.pc += 1;
        instruction.exec(self)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use bytecode::instructions;
    use bytecode::instructions::Instruction;

    #[bench]
    fn jmp_infinite_loop(b: &mut Bencher) {
        let mut i = Interpreter {
            pc: PC::new(vec![
                box Instruction::JMP(instructions::Jmp {
                    a: 0,
                    jump: -1,
                })
            ]),
        };
        b.iter(|| i.step())
    }
}