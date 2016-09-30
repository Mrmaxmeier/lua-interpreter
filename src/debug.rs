use parser::*;
use upvalues::Upvalues;
use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub struct Local {
    pub varname: String,
    pub startpc: u32,
    pub endpc: u32,
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.varname.fmt(f)
    }
}

impl Parsable for Local {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        Local {
            varname: String::parse(r),
            startpc: u32::parse(r),
            endpc: u32::parse(r),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct DebugData {
    pub line_info: Vec<u32>,
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
        let len_lineinfo = u32::parse(r);
        let line_info = (0..len_lineinfo)
            .map(|_| u32::parse(r))
            .collect();

        let len_locals = u32::parse(r);
        let locals = (0..len_locals)
            .map(|_| Local::parse(r))
            .collect();

        let len_upvalues = u32::parse(r);
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