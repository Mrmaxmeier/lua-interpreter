use bytecode::parser::*;
use types::Type;

/*
named!(pub parse_type< Box<Type> >, alt!(
    tag!(&[0]) => { |_| box Type::Nil } | // LUA_TNIL
    tag!(&[1, 0]) => { |_| box Type::Boolean(false) } | // LUA_TBOOLEAN
    tag!(&[1, 0]) => { |_| box Type::Boolean(true) } | // LUA_TBOOLEAN
    tag!(&[2]) => { |_| panic!("LUA_TLIGHTUSERDATA is not yet implemented") } | // LUA_TLIGHTUSERDATA
    tag!(&[3]) => { |_| panic!("LUA_TNUMBER is not yet implemented") } | // LUA_TNUMBER
    chain!(
        tag!(&[4]) ~
        s: parse_string,
        || {s}
    ) => { |s| box match s {
        Some(s) => Type::String(s),
        None => Type::Nil,
    }} | // LUA_TSTRING
    tag!(&[5]) => { |_| panic!("LUA_TTABLE is not yet implemented") } | // LUA_TTABLE
    tag!(&[6]) => { |_| panic!("LUA_TFUNCTION is not yet implemented") } | // LUA_TFUNCTION
    tag!(&[7]) => { |_| panic!("LUA_TUSERADTA is not yet implemented") } | // LUA_TUSERADTA
    tag!(&[8]) => { |_| panic!("LUA_TTHREAD is not yet implemented") } // LUA_TTHREAD
));
TODO: reimpl parse_type
*/