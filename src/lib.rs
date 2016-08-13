#![feature(box_syntax)]
#![feature(try_from)]

extern crate byteorder;

mod types;
pub use types::Type;
mod bytecode;
pub use bytecode::*;