use bytecode::parser::*;
use types::Type;

impl Parsable for Box<Type> {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        match r.read_byte() {
            0 => box Type::Nil,
            1 => match r.read_byte() {
                0 => box Type::Boolean(false),
                1 => box Type::Boolean(true),
                d => panic!("invalid boolean type {}", d)
            },
            2 => panic!("LUA_TLIGHTUSERDATA is not yet implemented"),
            3 => panic!("LUA_TNUMBER is not yet implemented"),
            4 => match r.parse_lua_string() {
                None => box Type::Nil,
                Some(string) => box Type::String(string),
            },
            5 => panic!("LUA_TTABLE is not yet implemented"),
            6 => panic!("LUA_TFUNCTION is not yet implemented"),
            7 => panic!("LUA_TUSERADTA is not yet implemented"),
            8 => panic!("LUA_TTHREAD is not yet implemented"),
            d => panic!("invalid type {}", d)
        }
    }
}

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