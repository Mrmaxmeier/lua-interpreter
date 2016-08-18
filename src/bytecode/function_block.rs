use bytecode::parser::*;
use bytecode::code::Code;
use bytecode::constants::Constants;
use bytecode::upvalues::Upvalues;
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
    pub protos: Vec<FunctionBlock>,
    pub upvalues: Upvalues,
    pub debug: Debug
}

impl FunctionBlock {
    fn propagate_source(&mut self, name: Option<String>) {
        for proto in &mut self.protos {
            proto.propagate_source(name.clone());
        }
        if self.source_name.is_none() {
            self.source_name = name;
        }
    }
}

impl Parsable for FunctionBlock {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let source_name = r.parse_lua_string();
        println!("source_name: {:?}", source_name);
        let lines = (u32::parse(r) as usize, u32::parse(r) as usize);
        let params = u8::parse(r);
        r.assert_byte(2); // is_vararg
        let stack_size = u8::parse(r);
        println!("stack_size: {:?}", stack_size);

        let code = Code::parse(r);
        println!("code {:#?}", code);
        let constants = Constants::parse(r);
        println!("constants {:#?}", constants);
        let mut upvalues = Upvalues::parse(r);
        println!("upvalues {:#?}", upvalues);
        let len_protos = u32::parse(r);
        println!("parsing {} protos (subblocks)", len_protos);
        let mut protos = (0..len_protos)
            .map(|_| FunctionBlock::parse(r))
            .collect::<Vec<_>>();
        for proto in &mut protos {
            proto.propagate_source(source_name.clone());
        }
        let debug = Debug::parse(r);
        if let Some(ref debug_data) = debug {
            debug_data.update_upvalues(&mut upvalues);
        }

        FunctionBlock {
            source_name: source_name,
            lines: lines,
            amount_parameters: params,
            stack_size: stack_size,
            instructions: code,
            constants: constants,
            upvalues: upvalues,
            protos: protos,
            debug: debug
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
    use bytecode::upvalues::Upvalue;
    use bytecode::parser::{Parsable, ReadExt};

    #[test]
    fn parses_assignment() {
        let all = include_bytes!("../../fixtures/assignment");
        let mut reader = Cursor::new(all.to_vec());
        Header::parse(&mut reader);
        reader.read_byte(); // skip count of upvalues
        assert_eq!(34, reader.position());

        let result = FunctionBlock::parse(&mut reader);
        println!("result: {:#?}\n", result);

        assert_eq!(result.source_name, Some("@assignment.lua".to_owned()));
        assert_eq!(result.lines, (0, 0));
        assert_eq!(result.stack_size, 2);
        assert_eq!(result.instructions, vec![
            box Instruction::LOADK(instructions::LoadK {local: 0, constant: 0}),
            box Instruction::RETURN(instructions::Return {a: 0, b: 0}),
        ]);
        assert_eq!(result.constants, vec![
            box Type::String("zweiundvierzig".into())
        ]);
        assert_eq!(result.upvalues, vec![
            Upvalue {
                name: Some("_ENV".into()),
                instack: 1,
                idx: 0
            }
        ]);
    }
}
