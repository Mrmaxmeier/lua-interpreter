use bytecode::header::Header;
use bytecode::function_block::FunctionBlock;
use bytecode::parser::*;

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


#[cfg(test)]
mod tests {
    use super::*;
    use bytecode::function_block::FunctionBlock;
    use bytecode::debug::Debug;
    use types::Type;
    use std::io::Cursor;
    use bytecode::parser::Parsable;
    use bytecode::instructions;
    use bytecode::instructions::Instruction;

    #[test]
    fn parses_assignment() {
        let data = include_bytes!("../../fixtures/assignment");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_a_bunch_of_constants() {
        let data = include_bytes!("../../fixtures/a_bunch_of_constants");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_call_correctly() {
        let data = include_bytes!("../../fixtures/call");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;
        println!("result: {:#?}\n", result);
        assert_eq!(result.source_name.unwrap(), "@call.lua".to_owned());
        assert_eq!(result.constants, vec![
            box Type::String("print".into()),
            box Type::String("value".into())
        ]);
        // TODO: check instructions
        // assert_eq!(result.instructions, vec![]);
    }

    #[test]
    fn parses_block_correctly() {
        let data = include_bytes!("../../fixtures/block");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec()));
        let expected = FunctionBlock {
            source_name: Some("@block.lua".into()),
            lines: (0, 0),
            amount_parameters: 0,
            stack_size: 2,
            instructions: vec![
                box Instruction::RETURN(instructions::Return{a: 0, b: 0})
            ],
            constants: vec![],
        // DEBUG DATA
            protos: (),
            upvalues: (),
            debug: Debug::default()
        };


        println!("result: {:#?}\n", result.func);
        assert_eq!(result.func, expected);
    }
}
