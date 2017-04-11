use std::fmt::Debug;
use std::io::Write;
use config::Config;
use asm::ast::{Node, Block};
use asm;

// Process State Traits
//
pub trait ProcessState: Debug + Clone {}

pub trait ErrorState: ProcessState {
  fn write<W: Write>(self, out: &mut W) {
    write!(out, "{:?}\n", self).ok();
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
