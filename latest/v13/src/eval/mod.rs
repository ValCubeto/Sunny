mod tokenize;
use crate::ctx::Ctx;

pub fn eval(input: String, ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  debug!(tokens);
  todo!();
}
