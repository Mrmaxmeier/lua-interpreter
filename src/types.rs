use std::collections::HashMap;

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
