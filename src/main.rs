use std::process::exit;

#[macro_use]
extern crate clap;

extern crate solasm;
use solasm::process::Processor;
use solasm::config::{Config, Target};
use solasm::process::plan::{self, Plan};

fn main() {
  let args = clap_app!(solasm =>
    (version: "0.1.0")
    (author: "g. nicholas d'andrea <nick@gnidan.org>")
    (about: "EVM Assembly Language compiler")
    (@arg ast: --ast "Output formatted assembly")
    (@arg filename: -f --filename[FILE] "Read from file instead of stdin")
  )
      .get_matches();

  let mut config = Config::new();

  if args.is_present("ast") {
    config.target(Target::Assembly);
  }

  if args.is_present("filename") {
    config.source_file(args.value_of("filename").unwrap());
  }


  let result = Processor::new()
    .configure(config.clone())
    .and_then(plan::FormatAssembly::run);

  match result {
    Ok(_) => {exit(0)},
    Err(_) => {exit(1)},
  }

  // if result.is_err() {
  //   let err = result.err().unwrap();
  //   println!("Parse Error!\n{}", err);
  //   process::exit(1);
  // }

  // let ast = result.unwrap();
  // let mut out : BufWriter<_> = BufWriter::new(io::stdout());

  // if config.targets(Target::Assembly) {
  //   PrettyPrinter::print(&ast, &mut out);
  //   write!(&mut out, "\n").ok();
  // }
}
