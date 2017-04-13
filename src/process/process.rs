use std::io::{self, BufWriter};
use std::result::Result;
use self::Result::{Ok, Err};

use config::*;
use super::state::*;
use asm;

#[derive(Debug, Clone, Default)]
pub struct Processor<S: ProcessState> {
  pub state: S,
}

pub type ProcessResult<T, E> = Result<Processor<T>, Processor<E>>;

impl Processor<New> {
  pub fn new() -> Processor<New> {
    Processor { state: New, ..Default::default() }
  }
}

impl<S: HasAST> Processor<S> {
  pub fn target<'a, E: ErrorState>(self) -> ProcessResult<Done, E> {
    let config = self.clone().config();
    let ast = self.clone().ast();

    if config.clone().targets(Target::Assembly) {
      let mut out: BufWriter<_> = BufWriter::new(io::stdout());
      asm::pretty::PrettyPrinter::print(&ast, &mut out);
    }

    Ok(Processor { state: Done {} })
  }

  pub fn ast(self) -> asm::ast::Node<asm::ast::Block> {
    self.state.get_ast()
  }
}

impl<E: ErrorState> Processor<E> {
  pub fn err<'a>(self) -> ProcessResult<Done, Error> {
    let mut out: BufWriter<_> = BufWriter::new(io::stderr());
    self.state.write(&mut out);
    Err(Processor { state: Error {} })
  }
}
