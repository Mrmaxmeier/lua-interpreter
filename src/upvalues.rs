use parser::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UpvalueInfo {
    pub name: Option<String>,
    pub instack: bool,
    pub index: u8,
}


impl Parsable for UpvalueInfo {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        UpvalueInfo {
            name: None,
            instack: u8::parse(r) > 0,
            index: u8::parse(r),
        }
    }
}

pub type UpvalueInfos = Vec<UpvalueInfo>;

impl Parsable for UpvalueInfos {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let amount = u32::parse(r);
        (0..amount).map(|_| UpvalueInfo::parse(r)).collect()
    }
}
