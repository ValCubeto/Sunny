use std::path::PathBuf;
use std::sync::Mutex;
use crate::args::ParsedArgs;

pub static FILE: Mutex<String> = Mutex::new(String::new());
pub static CONTENTS: Mutex<String> = Mutex::new(String::new());

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
