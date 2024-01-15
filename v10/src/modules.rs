use std::rc::Rc;
use hashbrown::HashMap;
use crate::context::Context;

pub fn parse_module_with_name(file_name: Rc<str>, code: &str) -> Context {
  let mut ctx = Context::new(code);
  ctx
}