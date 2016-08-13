use bytecode::header::{Header, parse_header};
use bytecode::function_block::{FunctionBlock, parse_function};
use std::convert::TryFrom;
use nom::IResult;

#[derive(Debug, Clone, PartialEq)]
pub struct Bytecode {
    pub header: Header,
    pub upvalues: u8,
    pub func: FunctionBlock,
}

impl<'a> TryFrom<&'a [u8]> for Bytecode {
    type Err = String;
    fn try_from(data: &'a [u8]) -> Result<Bytecode, String> {
        match parse_bytecode(data) {
            IResult::Done(_, bytecode) => Ok(bytecode),
            IResult::Error(e) => Err(format!("error: {:?}", e)),
            IResult::Incomplete(n) => Err(format!("needed: {:?}", n))
        }
    }
}

named!(pub parse_bytecode<Bytecode>, chain!(
    h: parse_header     ~
    upvalues: take!(1)  ~
    func: parse_function,
    || { Bytecode {
        header: h,
        upvalues: upvalues[0],
        func: func,
    } }
));

#[cfg(test)]
mod tests {
    use super::*;
    use bytecode::header::Header;
    use bytecode::function_block::FunctionBlock;
    use bytecode::debug::Debug;
    use types::Type;
    use std::io::Cursor;
    use nom::{IResult, Needed};

    #[test]
    fn parses_assignment() {
        let data = include_bytes!("../../fixtures/assignment");
        let result = parse_bytecode(data);
        println!("result: {:#?}\n", result);
        assert!(false);
    }



    #[test]
    fn parses_call() {
        let data = include_bytes!("../../fixtures/call");
        let result = parse_bytecode(data);
        let expected_header = Header::default();
        let expected_function_block = FunctionBlock {
            source_name: Some("@call.lua".into()),
            lines: (0, 0),
            amount_parameters: 0,
        //  is_vararg: VarArgs.VARARG_DEFAULT,
            stack_size: 0,
            instructions: Vec::new(),
            constants: vec![box Type::String("value".into())],
        // DEBUG DATA
            protos: (),
            upvalues: (),
            debug: Debug::default()
        };
        let expected = Bytecode {
            header: expected_header,
            upvalues: 0,
            func: expected_function_block,
        };


        let remaining: &[u8] = &[];
        println!("result: {:#?}\n", result);
        assert_eq!(result, IResult::Done(remaining, expected));
    }

    #[test]
    fn parses_block() {
        let data = include_bytes!("../../fixtures/block");
        let result = parse_bytecode(data);
        let expected_header = Header::default();
        let expected_function_block = FunctionBlock {
            source_name: Some("@block.lua".into()),
            lines: (0, 0),
            amount_parameters: 0,
        //  is_vararg: VarArgs.VARARG_DEFAULT,
            stack_size: 0,
            instructions: Vec::new(),
            constants: vec![box Type::String("value".into())],
        // DEBUG DATA
            protos: (),
            upvalues: (),
            debug: Debug::default()
        };
        let expected = Bytecode {
            header: expected_header,
            upvalues: 0,
            func: expected_function_block,
        };


        let remaining: &[u8] = &[];
        println!("result: {:#?}\n", result);
        assert_eq!(result, IResult::Done(remaining, expected));
    }
}
