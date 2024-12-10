use std::path::PathBuf;
use std::env::current_dir;
use std::process::exit;
use crate::args::ParsedArgs;
use crate::strings::EXTENSION;
use crate::debug;
use crate::terminal::Stylize;

pub fn run(args: ParsedArgs) {
  let file_path = parse_file_path(args.input);
  debug!(file_path);
}

// TODO: if current_dir() errors, get the path from args.this
fn parse_file_path(input: String) -> PathBuf {
  if input.is_empty() {
    // TODO: read from Sunny.toml
    eprintln!("{}: No input file specified", "Argument error".error());
    exit(1);
  }
  let path = input.trim();
  let mut path = PathBuf::from(path);
  // main -> main.sny
  if !path.exists() {
    let mut path2 = path.clone();
    path2.set_extension(EXTENSION);
    if !path2.exists() {
      eprintln!("{}: File \"{}\" does not exist", "Argument error".error(), current_dir().unwrap().join(path).display());
      exit(1);
    }
    path.set_extension(EXTENSION);
  }
  if !path.is_file() {
    eprintln!("{}: \"{}\" is not a file", "Argument error".error(), current_dir().unwrap().join(path).display());
    exit(1);
  }
  path
}
