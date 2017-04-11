use std::fmt::Debug;
use std::io::{Write};
use process::config::*;
use asm::ast::{Node, Block};
use asm;

// Process State Traits
//
pub trait ProcessState: Debug + Clone {}

pub trait ErrorState: ProcessState {
  fn write<W: Write>(self, out: &mut W) {
    write!(out, "{:?}\n", self);
  }
}


pub trait ConfiguredState: ProcessState {
  fn unwrap_config(self) -> Config;
}

pub trait ParsedState: ConfiguredState {
  fn unwrap_ast(self) -> Node<Block>;
}

pub trait WroteOutputState: ParsedState {}


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

impl ConfiguredState for Parsed {
  fn unwrap_config(self) -> Config {
    self.config
  }
}

impl ParsedState for Parsed {
  fn unwrap_ast(self) -> Node<Block> {
    self.ast
  }
}


// ParseError
//
#[derive(Debug, Clone)]
pub struct ParseError {
  error: asm::grammar::ParseError
}

impl ParseError {
  pub fn new(error: asm::grammar::ParseError) -> ParseError {
    ParseError { error: error }
  }
}

impl ProcessState for ParseError {}

impl ErrorState for ParseError {
  fn write<W: Write>(self, out: &mut W) {
    write!(out, "ParseError: {}\n", self.error);
  }
}


// WroteAssembly
//
#[derive(Debug, Clone)]
pub struct WroteAssembly {
  config: Config,
  ast: Node<Block>,
}

impl WroteAssembly {
  pub fn new(ast: Node<Block>, config: Config) -> WroteAssembly {
    WroteAssembly {
      config: config,
      ast: ast,
    }
  }
}

impl ProcessState for WroteAssembly {}

impl ConfiguredState for WroteAssembly {
  fn unwrap_config(self) -> Config {
    self.config
  }
}

impl ParsedState for WroteAssembly {
  fn unwrap_ast(self) -> Node<Block> {
    self.ast
  }
}

impl WroteOutputState for WroteAssembly {}


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

impl ErrorState for Error {}
