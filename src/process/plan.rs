//! Module for assemble plan, to determine process steps
use process::state::*;
use process::process::{Processor, ProcessResult};
use process::state::HasConfig;

pub trait Plan<S, T, E>
  where S: ProcessState,
        T: ProcessState,
        E: ErrorState
{
  fn run(Processor<S>) -> ProcessResult<T, E>;
}

pub struct FormatAssembly {}

impl<S: HasConfig> Plan<S, Done, Error> for FormatAssembly {
  fn run(processor: Processor<S>) -> ProcessResult<Done, Error> {
    processor.parse()
      .and_then(|p| p.target())
      .or_else(|p| p.err())
  }
}

#[cfg(test)]
use config::Config;

#[test]
fn it_parses_correctly() {
  let mut config = Config::new();
  config.source_str("{ i }");

  let result = Processor::new()
    .configure(config.clone())
    .and_then(FormatAssembly::run);

  assert!(result.is_ok());
}

#[test]
fn it_errors_correctly() {
  let mut config = Config::new();
  config.source_str("{ ! }");

  let result = Processor::new()
    .configure(config.clone())
    .and_then(FormatAssembly::run);

  assert!(result.is_err());
}
