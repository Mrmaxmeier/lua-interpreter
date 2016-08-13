use bytecode::header::Header;
use bytecode::function_block::FunctionBlock;
use bytecode::parser::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct Bytecode {
    pub header: Header,
    pub upvalues: u8,
    pub func: FunctionBlock,
}

impl Parsable for Bytecode {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        Bytecode {
            header: Header::parse(r),
            upvalues: u8::parse(r),
            func: FunctionBlock::parse(r),
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for Bytecode {
    type Err = String;
    fn try_from(data: &'a [u8]) -> Result<Bytecode, String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytecode::header::Header;
    use bytecode::function_block::FunctionBlock;
    use bytecode::debug::Debug;
    use types::Type;
    use std::io::Cursor;
    use bytecode::parser::Parsable;

    #[test]
    fn parses_assignment() {
        let data = include_bytes!("../../fixtures/assignment");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        println!("result: {:#?}\n", result);
        assert!(false);
    }



    #[test]
    fn parses_call() {
        let data = include_bytes!("../../fixtures/call");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
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


        println!("result: {:#?}\n", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn parses_block() {
        let data = include_bytes!("../../fixtures/block");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
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


        println!("result: {:#?}\n", result);
        assert_eq!(result, expected);
    }
}
