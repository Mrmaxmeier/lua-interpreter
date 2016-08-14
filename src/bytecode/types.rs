use bytecode::parser::*;
use types::{Type, Number};

const LUA_TSHRSTR: u8 = 4;               // short strings
const LUA_TLNGSTR: u8 = (4 | (1 << 4));  // long strings

const LUA_TNUMFLT: u8 = 3;               // float numbers
const LUA_TNUMINT: u8 = (3 | (1 << 4));  // integer numbers

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
            LUA_TNUMFLT => box Type::Number(Number::Float(Float::parse(r))),
            LUA_TNUMINT => box Type::Number(Number::Integer(Integer::parse(r))),
            LUA_TSHRSTR | LUA_TLNGSTR => match r.parse_lua_string() {
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