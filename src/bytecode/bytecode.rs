use bytecode::parser::*;
use bytecode::header::{Header, parse_header};
use bytecode::function_block::{parse_function, FunctionBlock};

#[derive(Debug, Clone, PartialEq)]
pub struct Bytecode {
    header: Header,
    upvalues: u8,
}

named!(pub parse_bytecode<Bytecode>, chain!(
    h: parse_header     ~
    upvalues: take!(1) ~
    func: parse_function,
    || { Bytecode {
        header: h,
        upvalues: upvalues[0],
    } }
));

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use nom::{IResult, Needed};

    #[test]
    fn parses_assignment() {
        let data = include_bytes!("../../fixtures/assignment");
        let result = parse_bytecode(data);
        println!("result: {:?}\n", result);
        assert!(false);
    }
}
