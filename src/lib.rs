#![feature(box_syntax)]
#![feature(try_from)]

#[macro_use]
extern crate nom;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate enum_primitive;
extern crate num;


mod types;
pub use types::Type;
mod bytecode;
pub use bytecode::*;
mod compiler;
pub use compiler::*;