use std::fs::File;
use std::io::{self, Read, BufReader, BufWriter};
use std::result::{Result};
use self::Result::{Ok, Err};

use super::config::*;
use super::state::*;
use asm;

#[derive(Debug, Clone, Default)]
pub struct Processor<S: ProcessState> {
  state: S,
}

pub type ProcessResult<T, E> = Result<Processor<T>, Processor<E>>;

impl Processor<New> {
  pub fn new() -> Processor<New> {
    Processor { state: New, ..Default::default() }
  }
}

impl Processor<New> {
  pub fn configure<'a>(self, config: Config) -> ProcessResult<Configured, Error> {
    Ok(Processor { state: Configured::new(config) })
  }
}

impl<S: ConfiguredState> Processor<S> {
  pub fn parse<'a>(self) -> ProcessResult<Parsed, Error> {
    let config = self.clone().config();
    let buffer = self.read(config.clone());
    let result = asm::grammar::block(buffer.as_str());

    Ok(Processor { state: Parsed::new(result, config) })
  }

  pub fn config<'a>(self) -> Config {
    self.state.unwrap_config()
  }

  pub fn read<'a>(self, config: Config) -> String {
    match config {
      Config { source: Source::Input, .. } => {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).ok();
        buffer
      }
      Config { source: Source::File { filename }, .. } => {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut buffer = String::new();
        buf_reader.read_to_string(&mut buffer).ok();
        buffer
      }
      Config { source: Source::Literal { source }, .. } => source,
    }
  }
}

impl<S: ParseResultState> Processor<S> {
  pub fn target<'a>(self) -> ProcessResult<WroteAssembly, Error> {
    let config = self.clone().config();
    let result = self.clone().parse_result();

    if config.clone().targets(Target::Assembly) {
      match result {
        Ok(tree) => {
          let mut out: BufWriter<_> = BufWriter::new(io::stdout());
          asm::pretty::PrettyPrinter::print(&tree, &mut out);
        }
        Err(err) => {}
      }
    }

    Ok(Processor { state: WroteAssembly::new(self.clone().parse_result(), config) })
  }

  pub fn parse_result(self) -> asm::grammar::ParseResult<asm::ast::Node<asm::ast::Block>> {
    self.state.unwrap_parse_result()
  }
}

impl<S: WroteOutputState> Processor<S> {
  pub fn finish<'a>(self) -> ProcessResult<Done, Error> {
    Ok(Processor { state: Done {} })
  }
}

#[test]
fn it_parses_correctly() {
  let mut config = Config::new();
  config.source_str("{ i }");

  let result = Processor::new()
    .configure(config.clone())
    .parse()
    .parse_result();

  assert!(result.is_ok());
}

#[test]
fn it_errors_correctly() {
  let mut config = Config::new();
  config.source_str("{ ! }");

  let result = Processor::new()
    .configure(config.clone())
    .parse()
    .parse_result();

  assert!(result.is_err());
}
