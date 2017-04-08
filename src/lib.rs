#[cfg(not(feature="nightly"))]
pub mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

#[cfg(feature="nightly")]
pub mod grammar {
    #![feature(plugin)]
    #![plugin(peg_syntax_ext)]
    peg_file! modname("grammar.rustpeg");
}

pub mod ast;

pub fn parse(s: &str)
    -> std::result::Result<ast::Node<ast::Block>, grammar::ParseError > { grammar::block(s) }

