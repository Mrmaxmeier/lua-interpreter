use bytecode::parser::*;
use bytecode::code::parse_code;
use bytecode::constants::parse_constants;
use bytecode::upvalues::parse_upvalues;
use bytecode::protos::parse_protos;
use bytecode::debug::parse_debug;
use bytecode::instructions::Instruction;
use types::Type;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBlock {
    pub source_name: Option<String>,
    pub lines: (usize, usize),
    pub amount_upvalues: u8,
    pub amount_parameters: u8,
    // is_vararg: VarArgs,
    pub stack_size: u8,
    pub instructions: Vec<Box<Instruction>>,
    pub constants: Vec<Box<Type>>,
// DEBUG DATA
    pub source_line_positions: Vec<()>,
    pub locals: Vec<()>,
    pub upvalues: Vec<()>,
}

named!(pub parse_function<FunctionBlock>, chain!(
    source: parse_string       ~
    line_s: parse_int          ~
    line_e: parse_int          ~
    numparams: take!(1)       ~
    is_vararg: take!(1)       ~
    maxstacksize: take!(1)    ~
    code: parse_code           ~
    constants: parse_constants ~
    upvalues: parse_upvalues   ~
    protos: parse_protos       ~
    debug: parse_debug         ,
    || { FunctionBlock {
        source_name: source,
        lines: (line_s as usize, line_e as usize),

        amount_upvalues: 0,
        amount_parameters: numparams[0],
        // is_vararg: VARARG_DEFAULT,
        stack_size: maxstacksize[0],
        instructions: code,
        constants: constants,
    // DEBUG DATA
        source_line_positions: Vec::new(),
        locals: Vec::new(),
        upvalues: Vec::new(),
    } }
));


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use nom::{IResult, Needed};
    use types::Type;
    use bytecode::header::parse_header;
    use bytecode::instructions::Instruction;

    #[test]
    fn parses_assignment() {
        let all = include_bytes!("../../fixtures/assignment");
        let data = parse_header(all).unwrap().0;
        let data = &data[1..]; // skip count of upvalues
        assert_eq!(34, all.len() - data.len());

        let result = parse_function(data).unwrap().1;
        let expected = FunctionBlock {
            source_name: Some("@assignment.lua".into()),
            lines: (0, 0),
            amount_upvalues: 1,
            amount_parameters: 0,
        //  is_vararg: VarArgs.VARARG_DEFAULT,
            stack_size: 0,
            instructions: vec![
                box Instruction::LOADK,
                box Instruction::RETURN,
            ],
            constants: vec![box Type::String("zweiundvierzig".into())],
        // DEBUG DATA
            source_line_positions: Vec::new(),
            locals: Vec::new(),
            upvalues: Vec::new(),
        };
        println!("result: {:#?}\n", result);
        assert_eq!(result, expected);
    }
}
