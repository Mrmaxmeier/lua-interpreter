use bytecode::parser::*;
use bytecode::instructions::Instruction;

pub type Code = Vec<Instruction>;

impl Parsable for Code {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let size = u32::parse(r);
        println!("parsing {} instructions", size);
        (0..size).map(|_| Instruction::parse(r)).collect()
    }
}