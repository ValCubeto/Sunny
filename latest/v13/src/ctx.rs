use std::path::PathBuf;
use crate::args::ParsedArgs;

pub struct Ctx {
  pub cwd: PathBuf,
  pub args: ParsedArgs,
}

impl Ctx {
  pub fn new(cwd: PathBuf, args: ParsedArgs) -> Self {
    Ctx {
      cwd,
      args,
    }
  }
}
