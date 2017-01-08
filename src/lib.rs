#![feature(box_syntax)]
#![feature(ptr_eq)]
#![feature(test)]
extern crate test;

extern crate regex;
extern crate byteorder;
extern crate parking_lot;

#[macro_use] pub mod types;
pub mod function;
pub mod table;

pub mod interpreter;
pub mod stack;
pub mod instruction;
pub mod instructions;
pub mod parser;
pub mod bytecode;
pub mod header;
pub mod function_block;
pub mod code;
pub mod constants;
pub mod upvalues;
pub mod debug;
pub mod env;

// TODO: integrate lundump.c/`check_*` methods
// TODO: .map(|t| t.clone) => .cloned