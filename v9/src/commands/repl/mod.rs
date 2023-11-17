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

    let line = format!("{{{line}}}");

    // otherwise it will exist during the entire program
    drop(buf);

    let mut ctx = Context::new("<stdin>".into(), &line);
    ctx.parse_block();

    fn a(_b: u8) {}
    let _a: std::rc::Rc<dyn Fn(u8)> = std::rc::Rc::new(a);
    todo!();
  }
}
