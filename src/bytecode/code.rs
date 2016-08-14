use bytecode::parser::*;
use bytecode::instructions::Instruction;

pub type Code = Vec<Box<Instruction>>;

impl Parsable for Code {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let size = Integer::parse(r);
        println!("parsing {} instructions", size);
        (0..size).map(|_| box Instruction::parse(r)).collect()
    }
}

/*
named!(pub parse_code<Code>, chain!(
    size: parse_int ~
    instructions: count!(call!(
        parse_instruction
    ), size as usize),
    || { instructions }
));
TODO: reimpl parse_code
*/