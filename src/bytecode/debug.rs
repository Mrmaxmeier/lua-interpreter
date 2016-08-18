use bytecode::parser::*;
use bytecode::upvalues::Upvalues;


#[derive(Debug, PartialEq, Clone, Default)]
pub struct Local {
    varname: String,
    startpc: Integer,
    endpc: Integer,
}

impl Parsable for Local {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        Local {
            varname: String::parse(r),
            startpc: Integer::parse(r),
            endpc: Integer::parse(r),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DebugData {
    pub line_info: Vec<Integer>,
    pub locals: Vec<Local>,
    pub upvalue_names: Vec<String>,
}

impl DebugData {
    pub fn update_upvalues(&self, upvalues: &mut Upvalues) {
        for (ref mut u, s) in upvalues.iter_mut().zip(&self.upvalue_names) {
            u.name = Some(s.clone());
        }
    }
}

pub type Debug = Option<DebugData>;

impl Parsable for Debug {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let len_lineinfo = Integer::parse(r);
        let line_info = (0..len_lineinfo)
            .map(|_| Integer::parse(r))
            .collect();

        let len_locals = Integer::parse(r);
        let locals = (0..len_locals)
            .map(|_| Local::parse(r))
            .collect();

        let len_upvalues = Integer::parse(r);
        let upvalues = (0..len_upvalues)
            .map(|_| String::parse(r))
            .collect();

        if len_lineinfo == 0 && len_locals == 0 && len_upvalues == 0 {
            None
        } else {
            Some(DebugData {
                line_info: line_info,
                locals: locals,
                upvalue_names: upvalues,
            })
        }
    }
}