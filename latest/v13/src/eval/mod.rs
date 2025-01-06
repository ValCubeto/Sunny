pub mod tokenize;
pub mod parse;
use crate::ctx::Ctx;

pub fn eval(input: String, _ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  for token in tokens.iter() {
    println!("{token}");
  }
  let items = parse::parse(tokens);
  // debug!(items);
  for item in items {
    debug_display!(item);
  }
  internal_err!("parsed all items. What's next?");
}
