use std::{
  path::PathBuf,
  process::exit,
  env
};

use crate::{
  about::{ NAME, VERSION },
  aliases::Id,
  commands::{ help, repl, run, version },
  argument_error,
};

#[derive(Debug)]
pub struct ParsedArgs {
  pub exec_path: Option<Id>,
  pub flags: Vec<Id>,
  pub command: Id,
  pub args: Vec<Id>
}

pub fn parse_args() -> ParsedArgs {
  let mut argv = env::args();

  let exec_path: Option<Id> = match argv.next() {
    None => None,
    Some(string) => Some(Id::from(string))
  };

  if argv.len() == 0 {
    println!("{NAME} v{VERSION}");
    println!(
      "Usage: {:?} [flags]* [command]",
      exec_path.unwrap_or(NAME.to_lowercase().into())
    );
    exit(0);
  }

  let mut flags: Vec<Id> = Vec::new();

  let mut command;

  for arg in &mut argv {
    let arg: &str = arg.as_str();

    if !arg.starts_with('-') {
      command = arg;
      break;
    }

  }

  let command = match argv.next() {
    None => argument_error!(""),
    Some(string) => string.as_str()
  };

  let args = Vec::with_capacity(argv.len());
  for arg in &mut argv {
    args.push(Id::from(arg));
  };

  match command {
    "version" => version::main(),
    "run"     => run    ::main(),
    "repl"    => repl   ::main(),
    "help"    => help   ::main()
  }

  ParsedArgs {
    exec_path,
    flags,
    command: Id::from(command),
    args
  }
}