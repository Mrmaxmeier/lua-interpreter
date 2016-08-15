use bytecode::parser::*;
use bytecode::upvalues::Upvalues;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DebugData {
    pub line_info: (),
    pub locals: (),
    pub upvalues: (),
}

impl DebugData {
    pub fn update_upvalues(&self, upvalues: &mut Upvalues) {}
}

pub type Debug = Option<DebugData>;

impl Parsable for Debug {
    fn parse<R: Read + Sized>(_: &mut R) -> Self {
        None
    }
}

/*
named!(pub parse_debug<Debug>, chain!(
    n_lineinfo: parse_int ~
    lineinfo: count!(take!(1), n_lineinfo as usize),
    || { Debug {
        line_info: (),
        locals: (),
        upvalues: (),
    }}
));
TODO: reimpl parse_debug
*/