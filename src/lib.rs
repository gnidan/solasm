#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod asm;
pub use asm::grammar;
pub mod process;
