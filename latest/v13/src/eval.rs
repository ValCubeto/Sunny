pub mod tokenize;
pub mod parse;

use crate::ctx::Ctx;

pub fn eval(input: String, _ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  debug_msg!("Tokenizing done");
  debug_msg!("tokens = [");
  for (_, token) in tokens.iter() {
    println!("    {token:?}");
  }
  println!("]");
  let entities = parse::parse(tokens);
  // debug!(items);
  for item in entities {
    debug_display!(item);
  }
  internal_err!("parsing done");
}
