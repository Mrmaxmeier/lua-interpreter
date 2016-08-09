#![feature(box_syntax)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate bitflags;

mod types;
pub use types::Type;
mod bytecode;
pub use bytecode::*;
mod compiler;
pub use compiler::*;