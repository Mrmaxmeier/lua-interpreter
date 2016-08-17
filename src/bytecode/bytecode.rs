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
    use bytecode::upvalues::Upvalue;
    use std::io::Cursor;
    use bytecode::parser::Parsable;
    use bytecode::instructions;
    use bytecode::instructions::Instruction;
    use types::{Type, Number};

    #[test]
    fn parses_assignment() {
        let data = include_bytes!("../../fixtures/assignment");
        Bytecode::parse(&mut Cursor::new(data.to_vec()));
    }

    #[test]
    fn parses_a_bunch_of_constants_correctly() {
        let data = include_bytes!("../../fixtures/a_bunch_of_constants");
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;


        assert_eq!(result.source_name.unwrap(), "@a_bunch_of_constants.lua".to_owned());
        assert_eq!(result.constants, vec![
            box Type::Number(Number::Integer(42)),
            box Type::Number(Number::Float(-0.08333333333)),
            box Type::String("TSHRSTR".to_owned()),
            box Type::String(
                "TLNGSTR\n\
______________________________________50\n\
_____________________________________100\n\
_____________________________________150\n\
_____________________________________200\n\
_____________________________________250\n\
_____________________________________300".to_owned()
            )
        ]);
    }

    #[ignore]
    #[test]
    fn parses_gcd() {
        let data = include_bytes!("../../fixtures/gcd");
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
        let result = Bytecode::parse(&mut Cursor::new(data.to_vec())).func;

        println!("result: {:#?}\n", result);

        assert_eq!(result.source_name.unwrap(), "@block.lua".to_owned());
        assert_eq!(result.lines, (0, 0));
        assert_eq!(result.amount_parameters, 0);
        assert_eq!(result.stack_size, 2);
        assert_eq!(result.instructions, vec![
            box Instruction::RETURN(instructions::Return{a: 0, b: 0})
        ]);
        assert_eq!(result.constants, vec![]);
        assert_eq!(result.protos, vec![]);
        assert_eq!(result.upvalues, vec![
            Upvalue {
                name: Some("_ENV".to_owned()),
                instack: 1,
                idx: 0
            }
        ]);
        assert!(result.debug.is_some());
    }
}
