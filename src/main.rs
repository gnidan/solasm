use std::io::{self, Read, Write, BufWriter};
use std::process;

#[macro_use]
extern crate clap;

extern crate solasm;
use solasm::parse;
use solasm::ast::pretty::PrettyPrinter;

fn main() {
  let args = clap_app!(solasm =>
    (version: "0.1.0")
    (author: "g. nicholas d'andrea <nick@gnidan.org>")
    (about: "EVM Assembly Language compiler")
    (@arg ast: --ast "Output formatted assembly")
  ).get_matches();

  let output_ast = args.is_present("ast");

  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).ok();

  let result = parse(buffer.as_str());
  if result.is_err() {
    let err = result.err().unwrap();
    println!("Parse Error!\n{}", err);
    process::exit(1);
  }

  let ast = result.unwrap();
  let mut out : BufWriter<_> = BufWriter::new(io::stdout());

  if output_ast {
    PrettyPrinter::print(&ast, &mut out);
    write!(&mut out, "\n").ok();
  }
}
