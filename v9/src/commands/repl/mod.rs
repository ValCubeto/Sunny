use std::io::{
  stdout, stdin,
  Write as _
};
use crate::{
  about::NAME,
  args::ParsedArgs,
  debug, context::Context,
};

pub fn main(args: ParsedArgs) {
  debug!(args);
  loop {
    print!("{NAME}> ");

    // otherwise the program reads the line
    // and THEN prints the prompt
    if let Err(error) = stdout().flush() {
      eprintln!("failed to print the prompt. {error}");
    }

    let mut buf: String = String::new();
    if let Err(error) = stdin().read_line(&mut buf) {
      eprintln!("failed to read the line. {error}");
    }

    // I use String because I need
    // to index later
    let line: String = buf
      .trim()
      .to_string();

    // no longer needed
    drop(buf);

    if line.is_empty() {
      continue;
    }

    debug!(line);

    // let mut ctx = Context::new(line);

    todo!();
  }
}
