use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use parking_lot::Mutex;

use parser::*;

pub type SharedType = Arc<Mutex<Type>>;
pub type LuaTable = HashMap<String, Type>; 

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Nil,
    Boolean(bool),
    Number(Number),
    String(String),
    Table(LuaTable),
/*
    Function,
    UserData,
    Thread,
*/
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Nil => write!(f, "nil"),
            Type::Boolean(val) => write!(f, "{}", val),
            Type::String(ref val) => write!(f, "{:?}", val),
            Type::Number(ref num) => {
                match *num {
                    Number::Float(ref v) => write!(f, "{:?}", v),
                    Number::Integer(ref v) => write!(f, "{}", v)
                }
            },
            _ => unimplemented!(),
        }
    }
}

const LUA_TSHRSTR: u8 = 4;               // short strings
const LUA_TLNGSTR: u8 = (4 | (1 << 4));  // long strings

const LUA_TNUMFLT: u8 = 3;               // float numbers
const LUA_TNUMINT: u8 = (3 | (1 << 4));  // integer numbers

impl Parsable for Type {
    fn parse<R: Read + Sized>(r: &mut R) -> Self {
        let kind = r.read_byte();
        // println!("parsing constant: {:#X}", kind);
        match kind {
            0 => Type::Nil,
            1 => match r.read_byte() {
                0 => Type::Boolean(false),
                1 => Type::Boolean(true),
                d => panic!("invalid boolean type {}", d)
            },
            2 => panic!("LUA_TLIGHTUSERDATA is not yet implemented"),
            LUA_TNUMFLT => Type::Number(Number::Float(Float::parse(r))),
            LUA_TNUMINT => Type::Number(Number::Integer(Integer::parse(r))),
            LUA_TSHRSTR | LUA_TLNGSTR => match r.parse_lua_string() {
                None => Type::Nil,
                Some(string) => Type::String(string),
            },
            5 => panic!("LUA_TTABLE is not yet implemented"),
            6 => panic!("LUA_TFUNCTION is not yet implemented"),
            7 => panic!("LUA_TUSERADTA is not yet implemented"),
            8 => panic!("LUA_TTHREAD is not yet implemented"),
            d => panic!("invalid type {}", d)
        }
    }
}