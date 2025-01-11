use std::fs;
use std::path::{ Path, PathBuf };
use std::env::current_dir;
use crate::args::ParsedArgs;
use crate::ctx::{ Ctx, CONTENTS, FILE };
use crate::strings::EXTENSION;
use crate::eval::eval;

pub fn run(args: ParsedArgs) {
  let cwd = current_dir().unwrap();
  let file_path = parse_file_path(&cwd, &args.input);
  debug_msg!("Working with file {}", cwd.join(&file_path).display());
  unsafe {
    FILE = args.input.clone();
  }
  let ctx = Ctx::new(cwd, args);
  let contents = fs::read_to_string(file_path).unwrap();
  unsafe {
    CONTENTS = contents.trim_end_matches([' ', '\t']).to_owned();
    CONTENTS.push(' ');
  }
  eval(contents, ctx);
}

fn parse_file_path(cwd: &Path, input: &str) -> PathBuf {
  if input.is_empty() {
    // TODO: read from Sunny.toml
    argument_err!("No input file specified");
  }
  let path = input.trim();
  let mut path = PathBuf::from(path);
  if !path.exists() {
    let mut path2 = path.clone();
    // main -> main.sny
    path2.set_extension(EXTENSION);
    if !path2.exists() {
      argument_err!("file \"{}\" does not exist", cwd.join(path2).display());
    }
    path.set_extension(EXTENSION);
  }
  if !path.is_file() {
    argument_err!("\"{}\" is not a file", cwd.join(path).display());
  }
  path
}
