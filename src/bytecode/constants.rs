use bytecode::parser::*;
use types::Type;

pub type Constants = Vec<Box<Type>>;

impl Parsable for Constants {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let count = Integer::parse(r);
        unimplemented!()
    }
}

/*
named!(pub parse_constants<Constants>, chain!(
    count: parse_int ~
    data: count!(call!(parse_type), count as usize),
    || { data }
));
*/
