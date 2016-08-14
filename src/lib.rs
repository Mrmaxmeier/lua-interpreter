#![feature(box_syntax)]

extern crate byteorder;

mod types;
pub use types::Type;
mod bytecode;
pub use bytecode::*;