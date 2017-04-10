//! Module for assemble plan, to determine process steps
use process::state::*;
use process::process::Processor;

pub enum PlanResult {
  Success(Processor<Done>),
  Error(Processor<Error>),
}

pub trait Plan {
  fn run(Processor<New>) -> PlanResult;
}
