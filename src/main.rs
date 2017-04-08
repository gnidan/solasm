use std::io::{self, Write, BufWriter};
use std::process;

#[macro_use]
extern crate clap;

extern crate solasm;
use solasm::ast::pretty::PrettyPrinter;
use solasm::process::Processor;
use solasm::process::config::{Config, Target};

fn main() {
  let args = clap_app!(solasm =>
    (version: "0.1.0")
    (author: "g. nicholas d'andrea <nick@gnidan.org>")
    (about: "EVM Assembly Language compiler")
    (@arg ast: --ast "Output formatted assembly")
    (@arg filename: -f --filename[FILE] "Read from file instead of stdin")
  ).get_matches();

  let mut config = Config::new();

  if args.is_present("ast") {
    config.target(Target::Assembly);
  }

  if args.is_present("filename") {
    config.source_file(args.value_of("filename").unwrap());
  }

  let result = Processor::new()
    .configure(config.clone())
    .parse()
    .parse_result();

  if result.is_err() {
    let err = result.err().unwrap();
    println!("Parse Error!\n{}", err);
    process::exit(1);
  }

  let ast = result.unwrap();
  let mut out : BufWriter<_> = BufWriter::new(io::stdout());

  if config.targets(Target::Assembly) {
    PrettyPrinter::print(&ast, &mut out);
    write!(&mut out, "\n").ok();
  }
}
