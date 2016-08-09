use bytecode::parser::*;
use bytecode::instructions::{Instruction, parse_instruction};

pub type Code = Vec<Box<Instruction>>;

named!(pub parse_code<Code>, chain!(
    size: parse_int ~
    instructions: count!(call!(
        parse_instruction
    ), size as usize),
    || { instructions }
));


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use nom::{IResult, Needed};

    #[ignore]
    #[test]
    fn parses_assignment() {
        unimplemented!()
    }
}
