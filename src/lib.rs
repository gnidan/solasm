#![feature(plugin)]
#![plugin(peg_syntax_ext)]

peg_file! grammar("grammar.rustpeg");

pub mod ast;

pub fn parse(s: &str)
    -> std::result::Result<ast::Node<ast::Block>, grammar::ParseError > { grammar::block(s) }

