use std::fmt::Debug;
use std::io::Write;
use config::Config;
use asm::ast::{Node, Block};

// Process State Traits
//
pub trait ProcessState: Debug + Clone {}

pub trait ErrorState: ProcessState {
  fn write<W: Write>(self, out: &mut W) {
    write!(out, "{:?}\n", self).ok();
  }
}


pub trait HasConfig: ProcessState {
  fn get_config(self) -> Config;
}

pub trait HasSource: ProcessState {
  fn get_source(self) -> String;
}

pub trait HasAST: HasConfig {
  fn get_ast(self) -> Node<Block>;
}

// New
//
#[derive(Debug, Clone, Default)]
pub struct New;
impl ProcessState for New {}


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
