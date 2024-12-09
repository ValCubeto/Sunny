extern crate hashbrown;
use hashbrown::HashMap;
use std::env;

#[allow(unused)]
#[derive(Debug)]
pub struct ParsedArgs {
  /// The first argument, the name of the executable
  pub this: String,
  /// The second argument, the command
  pub command: String,
  /// All arguments starting with a dash
  pub input: String,
  /// The third argument, the input
  pub flags: HashMap<String, String>,
  /// The rest of the arguments
  pub args: Vec<String>,
}

impl ParsedArgs {
  #[inline]
  pub fn new(this: String, command: String, input: String, flags: HashMap<String, String>, args: Vec<String>) -> Self {
    ParsedArgs {
      this,
      command,
      input,
      flags,
      args,
    }
  }
}

pub fn parse() -> ParsedArgs {
  let mut argv = env::args();
  let mut args = Vec::new();
  let mut flags = HashMap::new();
  let this = argv.next().unwrap_or_else(|| panic!("Cannot read argv!!"));
  let Some(command) = argv.next() else {
    return ParsedArgs::new(this, String::new(), String::new(), flags, args);
  };
  let mut input = String::new();
  // Parse flags first to ensure that the input is not a flag
  while let Some(arg) = argv.next() {
    if !arg.starts_with("-") {
      input = arg;
      break;
    }
    let key = arg.trim_start_matches('-').to_owned();
    let value = argv.next().unwrap_or_default();
    flags.insert(key, value);
  }
  while let Some(arg) = argv.next() {
    if arg.starts_with("-") {
      if arg == "--" {
        break;
      }
      let key = arg.trim_start_matches('-').to_owned();
      let value = argv.next().unwrap_or_default();
      flags.insert(key, value);
      continue;
    }
    args.push(arg);
  }
  ParsedArgs::new(this, command, input, flags, args)
}
