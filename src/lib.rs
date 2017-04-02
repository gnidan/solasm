pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub mod ast;

pub fn parse(s: &str) -> std::result::Result<self::ast::Block, self::grammar::ParseError > { self::grammar::block(s) }

