
use {
  std::{
    io::{ stdout, stdin, Write as _ },
    path::PathBuf,
    process::exit,
    rc::Rc
  },
  crate::{
    about::{ NAME, VERSION },
    aliases::Id,
    internal_error, argument_error
  }
};

#[derive(Debug)]
pub struct ParsedArgs {
  pub exec_path: PathBuf,
  pub flags: Rc<[Id]>,
  pub main_path: PathBuf,
  pub args: Rc<[Id]>
}

pub fn parse_args() -> ParsedArgs {
  let mut raw_args = std::env::args();

  let exec_path: PathBuf = match raw_args.next() {
    None => internal_error!("argv is empty"),
    Some(string) => PathBuf::from(string)
  };

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
        println!();
        println!("-h | --help        Prints this message");
        println!("-v | --version     Prints the current {NAME} version");
        println!("     --repl        Starts the REPL");
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
    flags: Rc::from(flags.as_slice()),
    main_path: main_path.unwrap_or(PathBuf::from("<stdin>")),
    args: Rc::from(args.as_slice())
  }
}