use std::path::PathBuf;
use crate::aliases::Id;

#[derive(Debug)]
pub struct ParsedArgs {
  pub exec_path: PathBuf,
  pub flags: Vec<Id>,
  pub main_path: PathBuf,
  pub args: Vec<Id>
}