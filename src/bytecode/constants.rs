use bytecode::parser::*;
use types::Type;

type BoxedType = Box<Type>;

pub type Constants = Vec<Box<Type>>;

impl Parsable for Constants {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let count = Integer::parse(r);
        (0..count).map(|_| BoxedType::parse(r)).collect()
    }
}