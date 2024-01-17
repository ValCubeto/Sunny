use crate::context::Context;

pub fn parse_module(code: &str) -> Context {
  let mut ctx = Context::new(code);
  while ctx.cursor() < ctx.char_count {
    let word = ctx.expect_word();
    todo!("{word:?}");
  }
  ctx
}
