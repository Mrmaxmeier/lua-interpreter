#[macro_use]
extern crate nom;

mod types;
pub use types::Type;
mod interpreter;
pub use interpreter::Interpreter;
mod ast;
mod parser;