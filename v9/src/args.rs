use std::path::PathBuf;
use crate::aliases::Id;

#[derive(Debug)]
pub struct ParsedArgs {
  pub exec_path: PathBuf,
  pub command: Id,
  pub main_path: PathBuf,
  pub args: Vec<Id>
}

/* 
pub fn parse_args() -> ParsedArgs {
  let mut raw_args = std::env::args();
  println!("{NAME} v{VERSION}");
  let exec_path: Option<PathBuf> = raw_args.next();
  println!(
    "Usage: {} [flags]* [command]",
    exec_path.unwrap_or(NAME.to_lowercase().into())
  );

  let mut flags: Vec<Id> = Vec::new();

  let mut main_path = None;

  for arg in &mut raw_args {
    let flag: &str = arg.as_str();

    if !flag.starts_with('-') {
      main_path = Some(PathBuf::from(arg));
      break;
    }

    match flag {
      "-v" | "--version" => {
        println!("{VERSION}");
        exit(0);
      }
      "--repl" => {
        print!("w> ");

        // otherwise the program reads the line and then prints the prompt
        stdout()
          .flush()
          .expect("failed to print the prompt");

        let mut buf: String = String::new();
        stdin()
          .read_line(&mut buf)
          .expect("failed to read the line");

        let line: &str = buf.trim();
        // if line.is_empty() { continue; }

        println!("read: {line:?}");

        todo!();
      }
      "-h" | "--help" => {
        println!("{NAME} v{VERSION}");
        println!("Usage: {NAME:e"} [flags]* [command]);
        println!();
        println!("-h | --help        Prints this message");
        println!("-v | --version     Prints the current {NAME} version");
        println!("   | --repl        Starts a new REPL session");
        exit(0);
      }
      "--test" => flags.push(Id::from(flag)),
      _ => argument_error!("unknown flag {flag:?}")
    }
  }

  if main_path.is_none() {
    argument_error!("missing file path");
  }

  let mut args: Vec<Id> = Vec::new();

  for arg in &mut raw_args {
    args.push(Id::from(arg));
  }

  ParsedArgs {
    exec_path,
    flags,
    main_path: main_path.unwrap(),
    args
  }
} */
