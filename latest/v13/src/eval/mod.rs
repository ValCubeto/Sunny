pub mod tokenize;
pub mod parse;
use crate::ctx::Ctx;

pub fn eval(input: String, _ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  // for (pos, token) in tokens.iter() {
  //   debug_msg!("[{}:{}] {token}", pos.line, pos.column);
  // }
  debug_msg!("parsed tokens");
  let items = parse::parse(tokens);
  // debug!(items);
  for item in items {
    debug_display!(item);
  }
  internal_err!("parsed all items. What's next?");
}
