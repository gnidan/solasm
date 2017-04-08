#![feature(plugin)]
#![plugin(peg_syntax_ext)]

peg_file! __grammar("grammar.rustpeg");

pub mod grammar {
    pub use super::__grammar::*;
}

pub mod ast;

pub fn parse(s: &str)
    -> std::result::Result<ast::Node<ast::Block>, grammar::ParseError > { grammar::block(s) }
