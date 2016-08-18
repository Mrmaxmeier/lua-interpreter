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
pub mod types;

// TODO: integrate lundump.c/`check_*` methods