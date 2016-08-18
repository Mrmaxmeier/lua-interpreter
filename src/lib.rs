#![feature(box_syntax)]
#![feature(question_mark)]
#![feature(test)]
extern crate test;

extern crate regex;
extern crate byteorder;

mod types;
pub use types::Type;
mod bytecode;
pub use bytecode::*;