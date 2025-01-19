pub mod tokenize;
pub mod parse;

use crate::ctx::Ctx;

pub fn eval(input: String, _ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  debug_msg!("tokens = [");
  for (_, token) in tokens.iter() {
    println!("  {token:?}");
  }
  println!("]");
  let module = parse::parse_mod(tokens);
  // debug!(items);
  for item in module {
    debug_msg!("{item}");
  }
  internal_err!("parsing done");
}
