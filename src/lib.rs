#![feature(box_syntax)]
#![feature(test)]
extern crate test;

extern crate byteorder;

mod types;
pub use types::Type;
mod bytecode;
pub use bytecode::*;