use bytecode::parser::*;
use bytecode::instructions::Instruction;

pub type Code = Vec<Box<Instruction>>;

impl Parsable for Code {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let size = u32::parse(r);
        println!("parsing {} instructions", size);
        (0..size).map(|_| box Instruction::parse(r)).collect()
    }
}