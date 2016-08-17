use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Integer(i32),
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
