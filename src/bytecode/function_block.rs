use bytecode::parser::*;
use bytecode::code::Code;
use bytecode::constants::Constants;
use bytecode::upvalues::Upvalues;
use bytecode::protos::Protos;
use bytecode::debug::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBlock {
    pub source_name: Option<String>,
    pub lines: (usize, usize),
    pub amount_parameters: u8,
    // is_vararg: VarArgs,
    pub stack_size: u8,
    pub instructions: Code,
    pub constants: Constants,
// DEBUG DATA
    pub protos: (),
    pub upvalues: (),
    pub debug: Debug
}

impl Parsable for FunctionBlock {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let source_name = r.parse_lua_string();
        let lines = (Integer::parse(r) as usize, Integer::parse(r) as usize);
        let params = u8::parse(r);
        r.assert_byte(0b10); // is_vararg
        FunctionBlock {
            source_name: source_name,
            lines: lines,
            amount_parameters: params,
            stack_size: u8::parse(r),
            instructions: Code::parse(r),
            constants: Constants::parse(r),
        // DEBUG DATA
            upvalues: Upvalues::parse(r),
            protos: Protos::parse(r),
            debug: Debug::parse(r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use types::Type;
    use bytecode::header::Header;
    use bytecode::instructions;
    use bytecode::instructions::Instruction;
    use bytecode::debug::Debug;
    use bytecode::parser::{Parsable, ReadExt};

    #[test]
    fn parses_assignment() {
        let all = include_bytes!("../../fixtures/assignment");
        let mut reader = Cursor::new(all.to_vec());
        Header::parse(&mut reader);
        reader.read_byte(); // skip count of upvalues
        assert_eq!(34, reader.position());

        let result = FunctionBlock::parse(&mut reader);
        let expected = FunctionBlock {
            source_name: Some("@assignment.lua".into()),
            lines: (0, 0),
            amount_parameters: 0,
        //  is_vararg: VarArgs.VARARG_DEFAULT,
            stack_size: 2,
            instructions: vec![
                box Instruction::LOADK(instructions::LoadK {a: 0, b: 0}),
                box Instruction::RETURN(instructions::Return {a: 0, b: 0}),
            ],
            constants: vec![box Type::String("zweiundvierzig".into())],
        // DEBUG DATA
            protos: (),
            upvalues: (),
            debug: Debug::default()
        };
        println!("result: {:#?}\n", result);
        assert_eq!(result, expected);
    }
}
