use std::fs;
use std::path::{ Path, PathBuf };
use std::env::current_dir;
use crate::args::ParsedArgs;
use crate::ctx::{ Ctx, CONTENTS, FILE };
use crate::strings::EXTENSION;
use crate::eval::eval;

pub fn run(args: ParsedArgs) {
  let cwd = match current_dir() {
    Ok(cwd) => cwd,
    Err(why) => sys_err!("failed to get the current directory ({why})")
  };
  let file_path = parse_file_path(&cwd, &args.input);
  debug_msg!("Working with file {}", cwd.join(&file_path).display());
  unsafe {
    FILE = args.input.clone();
  }
  let ctx = Ctx::new(cwd, args);
  let contents = match fs::read_to_string(&file_path) {
    Ok(contents) => contents,
    Err(why) => sys_err!("failed to read file \"{}\" ({why})", file_path.display())
  };
  unsafe {
    CONTENTS = contents.trim_end_matches([' ', '\t']).to_owned();
    CONTENTS.push(' ');
  }
  eval(contents, ctx);
}

fn parse_file_path(cwd: &Path, input: &str) -> PathBuf {
  if input.is_empty() {
    debug_todo!("read from Sunny.toml");
    argument_err!("No input file specified");
  }
  let path = input.trim();
  let mut path = PathBuf::from(path);
  if !path.exists() {
    let mut path2 = path.clone();
    // main -> main.sny
    if path2.extension().is_some() {
      argument_err!("file \"{}\" does not exist", cwd.join(path2).display());
    }
    path2.set_extension(EXTENSION);
    if !path2.exists() {
      argument_err!("file \"{}\" does not exist", cwd.join(path).display());
    }
    path.set_extension(EXTENSION);
  }
  if !path.is_file() {
    argument_err!("\"{}\" is not a file", cwd.join(path).display());
  }
  path
}
