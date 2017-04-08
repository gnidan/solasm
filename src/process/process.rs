use std::fs::File;
use std::io::{self, Read, BufReader};

use super::config::*;
use super::state::*;
use super::super::ast;
use super::super::grammar;

#[derive(Debug, Clone, Default)]
pub struct Processor<S: ProcessState> {
  state: S,
}

impl Processor<New> {
  pub fn new() -> Processor<New> {
    Processor {
      state: New,
      .. Default::default()
    }
  }
}

impl Processor<New> {
  pub fn configure<'a>(self, config: Config) -> Processor<Configured> {
    Processor {
      state: Configured::new(config),
    }
  }
}

impl<S: ConfiguredState> Processor<S> {
  pub fn parse<'a>(self) -> Processor<Parsed> {
    let config = self.clone().config();
    let buffer = self.read(config.clone());
    let result = grammar::block(buffer.as_str());

    Processor {
      state: Parsed::new(result, config),
    }
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
      },
      Config { source: Source::File { filename }, .. } => {
        let file = File::open(filename).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut buffer = String::new();
        buf_reader.read_to_string(&mut buffer).ok();
        buffer
      },
    }
  }
}

impl<S: ParseResultState> Processor<S> {
  pub fn parse_result(self) -> grammar::ParseResult<ast::Node<ast::Block>> {
    self.state.unwrap_parse_result()
  }
}
