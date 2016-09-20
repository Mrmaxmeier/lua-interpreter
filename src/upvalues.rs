use parser::*;

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UpvalueIndex {
    pub index: i64
}

impl UpvalueIndex {
    pub fn load(val: usize) -> Self {
        UpvalueIndex {
            index:
            if val >= 0b1_0000_0000 {
                -(val as i64 & 0xFF)
            } else {
                val as i64
            }
        }
    }
}

impl From<usize> for UpvalueIndex {
    fn from(other: usize) -> Self {
        UpvalueIndex::load(other)
    }
}
