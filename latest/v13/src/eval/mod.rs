mod tokenize;

use crate::{ctx::Ctx, debug};

pub fn eval(input: String, ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  debug!(tokens);
  todo!();
}
