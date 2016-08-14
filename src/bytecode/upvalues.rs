use bytecode::parser::*;
pub type Upvalues = ();

impl Parsable for Upvalues {
    fn parse<R: Read + Sized>(_: &mut R) -> Self {
        // let amount = Integer::parse(r);
        () // TODO: impl Parsable for Upvalues
    }
}