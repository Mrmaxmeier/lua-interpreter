use std::collections::HashMap;

/*
There are eight basic types in Lua: nil, boolean, number, string, function, userdata, thread, and table.
The type nil has one single value, nil, whose main property is to be different from any other value; it usually represents the absence of a useful value. The type boolean has two values, false and true. Both nil and false make a condition false; any other value makes it true.
The type number represents both integer numbers and real (floating-point) numbers.
The type string represents immutable sequences of bytes. Lua is 8-bit clean: strings can contain any 8-bit value, including embedded zeros ('\0'). Lua is also encoding-agnostic; it makes no assumptions about the contents of a string.
*/

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
    Table(HashMap<String, Type>)    
/*
    Function,
    UserData,
    Thread,
*/
}
