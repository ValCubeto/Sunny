use std::path::PathBuf;
use crate::args::ParsedArgs;

pub static mut FILE: String = String::new();
pub static mut CONTENTS: String = String::new();

#[allow(unused)]
pub struct Ctx {
  pub cwd: PathBuf,
  pub args: ParsedArgs
}

impl Ctx {
  pub fn new(cwd: PathBuf, args: ParsedArgs) -> Self {
    Ctx {
      cwd,
      args
    }
  }
}
