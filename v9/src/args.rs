use std::{
  process::exit,
  env, path::Path
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

pub fn parse_args() {
  let mut argv = env::args();

  // if there's Some(path), converts that String into an Id
  let exec_path: Option<Id> = argv.next().map(Id::from);

  if argv.len() == 0 {
    println!("{NAME} v{VERSION}");
    // convert into a String again?
    let id = exec_path.map(|id| id.to_string()).unwrap_or(NAME.to_lowercase());
    let path = Path::new(&id).file_name();
    println!("Usage: {path:?} [flags]* [command]");
    exit(0);
  }

  let mut flags: Vec<Id> = Vec::new();

  for arg in &mut argv {
    let arg = Id::from(arg);

    if arg.starts_with('-') {
      flags.push(arg);
      continue;
    }

    let mut other_args = Vec::with_capacity(argv.len());
    for arg in &mut argv {
      other_args.push(Id::from(arg));
    };

    let parsed_args = ParsedArgs {
      exec_path,
      flags,
      command: Id::clone(&arg),
      args: other_args
    };
    match &*arg {
      "version" => version::main(),
      "help"    => help   ::main(),
      "run"     => run    ::main(parsed_args),
      "repl"    => repl   ::main(parsed_args),
      _ => argument_error!("unknown command {arg:?}")
    }
    break;
  }
}