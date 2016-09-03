#![feature(box_syntax)]
#![feature(question_mark)]
#![feature(test)]
extern crate test;

extern crate regex;
extern crate byteorder;

pub mod types;
pub mod interpreter;
pub mod instructions;
pub mod parser;
pub mod bytecode;
pub mod header;
pub mod function_block;
pub mod code;
pub mod constants;
pub mod upvalues;
pub mod debug;

// TODO: integrate lundump.c/`check_*` methods