use std::io::{self, BufReader, Write, Read};
use std::fs::File;
use process::{Processor, ProcessResult};
use process::state::{New, Error, ProcessState, ErrorState, HasConfig, HasAST};
use config::{Config, Source};
use asm;
use asm::ast::{Node, Block};

// Parsed
//
#[derive(Debug, Clone)]
pub struct Parsed {
  config: Config,
  ast: Node<Block>,
}

impl Parsed {
  pub fn new(ast: Node<Block>, config: Config) -> Parsed {
    Parsed {
      config: config,
      ast: ast,
    }
  }
}

impl ProcessState for Parsed {}

impl HasConfig for Parsed {
  fn unwrap_config(self) -> Config {
    self.config
  }
}

impl HasAST for Parsed {
  fn unwrap_ast(self) -> Node<Block> {
    self.ast
  }
}


// ParseError
//
#[derive(Debug, Clone)]
pub struct ParseError {
  error: asm::grammar::ParseError,
}

impl ParseError {
  pub fn new(error: asm::grammar::ParseError) -> ParseError {
    ParseError { error: error }
  }
}

impl ProcessState for ParseError {}

impl ErrorState for ParseError {
  fn write<W: Write>(self, out: &mut W) {
    write!(out, "ParseError: {}\n", self.error).ok();
  }
}


impl<S: HasConfig> Processor<S> {
  pub fn parse<'a>(self) -> ProcessResult<Parsed, ParseError> {
    let config = self.clone().config();
    let buffer = self.read(config.clone());
    let result = asm::grammar::block(buffer.as_str());

    result.and_then(|ast| Ok(Processor { state: Parsed::new(ast, config) }))
      .or_else(|err| Err(Processor { state: ParseError::new(err) }))
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
