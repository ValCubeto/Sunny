pub mod tokenize;
pub mod parse;

use crate::ctx::Ctx;

pub fn eval(input: String, _ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  for token in tokens.iter() {
    debug!(token);
  }
  let module = parse::parse_mod(tokens);
  for item in module {
    debug_msg!("{item}");
  }
  internal_err!("parsing done");
}
