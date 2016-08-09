use bytecode::parser::*;
named!(pub parse_debug<()>, chain!(
    amount: parse_int,
    || {(/* TODO: implement parse_debug */)}
));
