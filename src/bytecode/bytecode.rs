use bytecode::header::{Header, parse_header};
use bytecode::function_block::{parse_function};

#[derive(Debug, Clone, PartialEq)]
pub struct Bytecode {
    header: Header,
    upvalues: u8,
}

named!(pub parse_bytecode<Bytecode>, chain!(
    h: parse_header     ~
    upvalues: take!(1)  ~
    func: parse_function,
    || { Bytecode {
        header: h,
        upvalues: upvalues[0],
    } }
));

#[cfg(test)]
mod tests {
    use super::*;
    use bytecode::header::Header;
    use bytecode::function_block::FunctionBlock;
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
            amount_upvalues: 1,
            amount_parameters: 0,
        //  is_vararg: VarArgs.VARARG_DEFAULT,
            stack_size: 0,
            instructions: Vec::new(),
            constants: vec![box Type::String("value".into())],
        // DEBUG DATA
            source_line_positions: Vec::new(),
            locals: Vec::new(),
            upvalues: Vec::new(),
        };
        let expected = Bytecode {
            header: expected_header,
            upvalues: 0
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
            amount_upvalues: 1,
            amount_parameters: 0,
        //  is_vararg: VarArgs.VARARG_DEFAULT,
            stack_size: 0,
            instructions: Vec::new(),
            constants: vec![box Type::String("value".into())],
        // DEBUG DATA
            source_line_positions: Vec::new(),
            locals: Vec::new(),
            upvalues: Vec::new(),
        };
        let expected = Bytecode {
            header: expected_header,
            upvalues: 0
        };


        let remaining: &[u8] = &[];
        println!("result: {:#?}\n", result);
        assert_eq!(result, IResult::Done(remaining, expected));
    }
}
