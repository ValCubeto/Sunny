use hashbrown::HashMap;
use std::env;

type Flags = HashMap<String, String>;

#[allow(unused)]
#[derive(Debug)]
pub struct ParsedArgs {
  /// The first argument, the name of the executable
  pub this: String,
  /// The second argument, the command
  pub command: String,
  /// The third argument, the input
  pub input: String,
  /// All arguments starting with a dash
  pub flags: Flags,
  /// The rest of the arguments
  pub args: Vec<String>,
}

impl ParsedArgs {
  #[inline]
  pub fn new(this: String, command: String, input: String, flags: Flags, args: Vec<String>) -> Self {
    ParsedArgs {
      this,
      command,
      input,
      flags,
      args,
    }
  }
}

const VALID_FLAGS: [&str; 2] = [
  "no-color",
  "debug",
];

pub fn parse() -> ParsedArgs {
  let mut argv = env::args();
  let mut args = Vec::new();
  let mut flags = HashMap::new();
  let this = argv.next().unwrap_or_else(|| {
    panic!("argv is empty!");
  });
  let mut input = String::new();
  let command = match argv.next() {
    Some(cmd) => cmd,
    None => return ParsedArgs::new(this, String::new(), input, flags, args)
  };

  // Parse flags first to ensure that the input is not a flag
  for arg in &mut argv {
    if !arg.starts_with('-') {
      input = arg;
      break;
    }
    if arg == "--" {
      break;
    }
    add_flag(arg, &mut flags);
  }
  for arg in &mut argv {
    if !arg.starts_with('-') {
      args.push(arg);
      break;
    }
    add_flag(arg, &mut flags);
  }
  for arg in argv {
    args.push(arg);
  }
  ParsedArgs::new(this, command, input, flags, args)
}

fn add_flag(arg: String, flags: &mut Flags) {
  let mut split = arg.trim_start_matches('-').splitn(2, '=');
  let key = split.next().unwrap();
  if !VALID_FLAGS.contains(&key) {
    // Cannot use standard argument error yet because of
    // possible modifiers on the flags themselves
    eprintln!("Warning: invalid flag {key:?}");
  }
  let value = split.next().unwrap_or_default().to_owned();
  flags.insert(key.to_owned(), value);
}
