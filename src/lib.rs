pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub mod ast;

pub fn parse(s: &str)
    -> std::result::Result<ast::Node<ast::Block>, grammar::ParseError > { grammar::block(s) }

