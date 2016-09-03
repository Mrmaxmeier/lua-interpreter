use bytecode::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Upvalue {
    pub name: Option<String>,
    pub instack: bool,
    pub index: u8,
}


impl Parsable for Upvalue {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        Upvalue {
            name: None,
            instack: u8::parse(r) > 0,
            index: u8::parse(r),
        }
    }
}

pub type Upvalues = Vec<Upvalue>;

impl Parsable for Upvalues {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let amount = u32::parse(r);
        (0..amount).map(|_| Upvalue::parse(r)).collect()
    }
}