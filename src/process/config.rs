use std::collections::HashSet;

#[derive(Debug, Clone, Default)]
pub struct Config {
  pub source: Source,
  pub targets: HashSet<Target>,
}

impl Config {
  pub fn new() -> Config {
    Config {
      ..Default::default()
    }
  }

  pub fn source_file<'a>(&'a mut self, filename: &str) -> &'a mut Config {
    self.source = Source::File { filename: String::from(filename) };
    self
  }

  pub fn source_stdin<'a>(&'a mut self) -> &'a mut Config {
    self.source = Source::Input;
    self
  }

  pub fn target<'a>(&'a mut self, target: Target) -> &'a mut Config {
    self.targets.insert(target);
    self
  }

  pub fn targets(self, target: Target) -> bool {
    self.targets.contains(&target)
  }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Source {
  Input,
  File { filename: String },
}

impl Default for Source {
  fn default() -> Self { Source::Input }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Target {
  Assembly,
}
