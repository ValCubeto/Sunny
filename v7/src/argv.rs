use {
  std::{
    path::PathBuf,
    process::exit 
  },
  crate::{
    about::{ NAME, VERSION },
    aliases::Id,
    { internal_error, argument_error }
  }
};

#[derive(Debug)]
pub struct ParsedArgs {
  pub exec_path: PathBuf,
  pub flags: Vec<Id>,
  pub main_path: PathBuf,
  pub args: Vec<Id>
}

pub fn parse_args() -> ParsedArgs {
  let mut raw_args = std::env::args_os().enumerate();

  let exec_path: PathBuf = match raw_args.next() {
    None => internal_error!("argv is empty"),
    Some((_i, os_string)) => PathBuf::from(os_string)
  };

  let mut flags: Vec<Id> = Vec::new();

  let mut main_path = None;

  for (i, raw_arg) in &mut raw_args {
    let flag: &str = match raw_arg.to_str() {
      None => argument_error!("the arguments must be valid Unicode. Failed at position {}", i + 1),
      Some(flag) => flag
    };

    if !flag.starts_with('-') {
      main_path = Some(PathBuf::from(raw_arg));
      break;
    }

    match flag {
      "-v" | "--version" => {
        let arg_count = raw_args.len();
        if arg_count > 0 {
          println!("# unused {} extra arguments", arg_count);
        }
        println!("{NAME} {VERSION}");
        exit(0);
      }
      "--test" => flags.push(Id::from(flag)),
      _ => argument_error!("invalid flag {:?}", flag)
    }
  }

  if main_path.is_none() {
    // TODO: interactive mode
    argument_error!("missing file path");
  }

  let mut args: Vec<Id> = Vec::new();

  for (i, arg) in &mut raw_args {
    args.push(match arg.to_str() {
      None => argument_error!("the arguments must be valid Unicode. Failed at position {}", i + 1),
      Some(arg) => Id::from(arg),
    })
  }

  ParsedArgs {
    exec_path,
    flags,
    main_path: main_path.unwrap(), // unwrap_or(PathBuf::from("<stdin>"))
    args
  }
}