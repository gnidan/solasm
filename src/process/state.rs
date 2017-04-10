use std::fmt::Debug;
use process::config::*;
use asm::ast::{Node, Block};
use asm::grammar::ParseResult;

// Process State Traits
//
pub trait ProcessState: Debug + Clone {}

pub trait ConfiguredState: ProcessState {
  fn unwrap_config(self) -> Config;
}

pub trait ParseResultState: ConfiguredState {
  fn unwrap_parse_result(self) -> ParseResult<Node<Block>>;
}


// New
//
#[derive(Debug, Clone, Default)]
pub struct New;
impl ProcessState for New {}


// Configured
//
#[derive(Debug, Clone, Default)]
pub struct Configured {
  config: Config,
}

impl Configured {
  pub fn new(config: Config) -> Configured {
    Configured { config: config }
  }
}

impl ProcessState for Configured {}

impl ConfiguredState for Configured {
  fn unwrap_config(self) -> Config {
    self.config
  }
}


// Parsed
//
#[derive(Debug, Clone)]
pub struct Parsed {
  config: Config,
  result: ParseResult<Node<Block>>,
}

impl Parsed {
  pub fn new(result: ParseResult<Node<Block>>, config: Config) -> Parsed {
    Parsed {
      config: config,
      result: result,
    }
  }
}

impl ProcessState for Parsed {}

impl ConfiguredState for Parsed {
  fn unwrap_config(self) -> Config {
    self.config
  }
}

impl ParseResultState for Parsed {
  fn unwrap_parse_result(self) -> ParseResult<Node<Block>> {
    self.result
  }
}


// WroteAssembly
//
#[derive(Debug, Clone)]
pub struct WroteAssembly {
  config: Config,
  result: ParseResult<Node<Block>>,
}

impl WroteAssembly {
  pub fn new(result: ParseResult<Node<Block>>, config: Config) -> WroteAssembly {
    WroteAssembly {
      config: config,
      result: result,
    }
  }
}

impl ProcessState for WroteAssembly {}

impl ConfiguredState for WroteAssembly {
  fn unwrap_config(self) -> Config {
    self.config
  }
}

impl ParseResultState for WroteAssembly {
  fn unwrap_parse_result(self) -> ParseResult<Node<Block>> {
    self.result
  }
}


// Done
//
#[derive(Debug, Clone)]
pub struct Done {}

impl Done {
  pub fn new() -> Done {
    Done {}
  }
}

impl ProcessState for Done {}


// Error
//
#[derive(Debug, Clone)]
pub struct Error {}

impl Error {
  pub fn new() -> Error {
    Error {}
  }
}

impl ProcessState for Error {}
