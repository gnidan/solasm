#[macro_use]
extern crate log;

pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub mod ast;
