use process::{Processor, ProcessResult};
use process::state::{New, Error, ProcessState, HasConfig};
use config::Config;

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

impl HasConfig for Configured {
  fn get_config(self) -> Config {
    self.config
  }
}

impl Processor<New> {
  pub fn configure<'a>(self, config: Config) -> ProcessResult<Configured, Error> {
    Ok(Processor { state: Configured::new(config) })
  }
}
