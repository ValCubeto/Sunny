use std::io::{
  stdout, stdin,
  Write as _
};
use crate::{
  about::NAME,
  args::ParsedArgs,
  context::Context,
  debug, debug_expr,
};

pub fn main(args: ParsedArgs) {
  debug!("{args:?}");
  loop {
    print!("{NAME}> ");

    // otherwise the program reads the line
    // and THEN prints the prompt
    if let Err(why) = stdout().flush() {
      eprintln!("failed to print the prompt. {why}");
    }

    let mut buf = String::new();
    if let Err(why) = stdin().read_line(&mut buf) {
      eprintln!("failed to read the line. {why}");
    }

    let line = buf.trim();

    if line.is_empty() {
      continue;
    }

    debug_expr!(line);

    let mut code = String::with_capacity(line.len() + 2);
    code.push('{');
    code.push_str(line);
    code.push('}');
    let line = code.as_str();

    let mut ctx = Context::new("<stdin>".into(), line);
    ctx.parse_block();

    todo!();
  }
}
