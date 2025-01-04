use std::path::{ Path, PathBuf };
use std::env::current_dir;
use std::process::exit;
use crate::args::ParsedArgs;
use crate::ctx::Ctx;
use crate::strings::EXTENSION;
use crate::eval::eval;
use crate::terminal::Stylize;

pub fn run(args: ParsedArgs) {
  let cwd = current_dir().unwrap();
  let file_path = parse_file_path(&cwd, &args.input);
  debug_msg!("Working with file {}", cwd.join(&file_path).display());
  let ctx = Ctx::new(cwd, args);
  let contents = std::fs::read_to_string(file_path).unwrap();
  debug!(contents);
  eval(contents, ctx);
}

fn parse_file_path(cwd: &Path, input: &str) -> PathBuf {
  if input.is_empty() {
    // TODO: read from Sunny.toml
    eprintln!("{}: No input file specified", "Argument error".error());
    exit(1);
  }
  let path = input.trim();
  let mut path = PathBuf::from(path);
  if !path.exists() {
    let mut path2 = path.clone();
    // main -> main.sny
    path2.set_extension(EXTENSION);
    if !path2.exists() {
      eprintln!("{}: File \"{}\" does not exist", "Argument error".error(), cwd.join(path2).display());
      exit(1);
    }
    path.set_extension(EXTENSION);
  }
  if !path.is_file() {
    eprintln!("{}: \"{}\" is not a file", "Argument error".error(), cwd.join(path).display());
    exit(1);
  }
  path
}
