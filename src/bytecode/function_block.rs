use bytecode::parser::*;
use bytecode::code::parse_code;
use bytecode::constants::parse_constants;
use bytecode::upvalues::parse_upvalues;
use bytecode::protos::parse_protos;
use bytecode::debug::parse_debug;
use bytecode::instructions::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionBlock {
    source_name: Option<String>,
    lines: (usize, usize),
    amount_upvalues: u8,
    amount_parameters: u8,
    // is_vararg: VarArgs,
    stack_size: u8,
    instructions: Vec<Box<Instruction>>,
    constants: Vec<()>,
// DEBUG DATA
    source_line_positions: Vec<()>,
    locals: Vec<()>,
    upvalues: Vec<()>,
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
        constants: Vec::new(),
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

    #[test]
    fn parses_assignment() {
        let start = 29 + 5; // FIXME
        let data = &include_bytes!("../../fixtures/assignment")[start..];
        let result = parse_function(data);
        let expected = FunctionBlock {
            source_name: Some("@assignment.lua".into()),
            lines: (0, 0),
            amount_upvalues: 1,
            amount_parameters: 0,
        //  is_vararg: VarArgs.VARARG_DEFAULT,
            stack_size: 0,
            instructions: Vec::new(),
            constants: Vec::new(),
        // DEBUG DATA
            source_line_positions: Vec::new(),
            locals: Vec::new(),
            upvalues: Vec::new(),
        };
        let remaining = &data[24..];
        println!("result: {:#?}\n", result);
        assert_eq!(result, IResult::Done(remaining, expected));
    }
}
