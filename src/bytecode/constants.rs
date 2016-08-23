use bytecode::parser::*;
use types::Type;

pub type Constants = Vec<Type>;

impl Parsable for Constants {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let count = u32::parse(r);
        println!("parsing {} constants", count);
        (0..count).map(|_| Type::parse(r)).collect()
    }
}