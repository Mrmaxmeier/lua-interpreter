use bytecode::parser::*;

named!(pub parse_upvalues<()>, chain!(
    amount: parse_int,
    || {(/* TODO: implement parse_upvalues */)}
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
