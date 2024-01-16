use std::rc::Rc;
use crate::context::Context;

pub fn parse_module_with_name(file_name: Rc<str>, code: &str) -> Context {
  let mut ctx = Context::new(code);
  while ctx.cursor < ctx.char_count {
    let word = ctx.expect_word();
    todo!("{word:?}");
  }
  ctx
}
