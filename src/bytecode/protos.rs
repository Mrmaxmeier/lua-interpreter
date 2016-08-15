use bytecode::parser::*;

pub type Protos = ();

impl Parsable for Protos {
     fn parse<R: Read + Sized>(r: &mut R) -> Self {
         ()
     }
}

/*
named!(pub parse_protos<()>, chain!(
    amount: parse_int,
    || {(/* TODO: implement parse_protos */)}
));
*/