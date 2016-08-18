use std::collections::HashMap;
use std::fmt;

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
    Table(HashMap<String, Box<Type>>),
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

