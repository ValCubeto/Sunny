mod tokenize;
mod parse;
use crate::ctx::Ctx;

pub fn eval(input: String, _ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  // debug!(tokens);
  let items = parse::parse(tokens);
  // debug!(items);
  for item in items {
    debug!(item);
  }
  internal_err!("parsed all items. What's next?");
}
