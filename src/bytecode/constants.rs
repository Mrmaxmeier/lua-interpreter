use bytecode::parser::*;
use bytecode::types::parse_type;
use types::Type;

pub type Constants = Vec<Box<Type>>;

named!(pub parse_constants<Constants>, chain!(
    count: parse_int ~
    data: count!(call!(parse_type), count as usize),
    || { data }
));


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use nom::{IResult, Needed};

    #[ignore]
    #[test]
    fn parses_assignment() {
        unimplemented!()
    }
}
