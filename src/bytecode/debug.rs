use bytecode::parser::*;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Debug {
    pub line_info: (),
    pub locals: (),
    pub upvalues: (),
}

named!(pub parse_debug<Debug>, chain!(
    n_lineinfo: parse_int ~
    lineinfo: count!(take!(1), n_lineinfo as usize),
    || { Debug {
        line_info: (),
        locals: (),
        upvalues: (),
    }}
));
