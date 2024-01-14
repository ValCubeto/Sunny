use std::rc::Rc;
use hashbrown::HashMap;
use crate::context::Context;

pub fn parse_module_with_name(ctx: &mut Context, name: Rc<str>) -> (Rc<str>, Value) {
  println!("parse_module_with_name(ctx, {name:?})");
  let mut module = Module::new(name.clone());
  ctx.skip_spaces();
  while ctx.current != '}' {
    let word = ctx.expect_word();
    dbg!(&word);
    let (name, value) = match word.as_str() {
      "mod" => parse_module(ctx),
      | "fun"
      | "class"
      | "struct"
      | "enum"
      | "flagset"
      | "use"
      | "mixin"
      | "impl"
      | "const"
      | "test"
        => syn_error!("{word:?} not implemented yet"),
      _ => syn_error!("unexpected word {word:?} here")
    };
    module.set(name, value);
    ctx.skip_spaces();
  }
  println!("{name:?}");
  ctx.debug();
  (name, Value::Module(module))
}

#[inline(always)]
pub fn parse_module(ctx: &mut Context) -> (Rc<str>, Value) {
  ctx.skip_spaces();
  let name: Rc<str> = Rc::from(ctx.expect_word());
  ctx.expect_token('{');
  ctx.next_char();
  parse_module_with_name(ctx, name.clone())
}

pub struct Module {
  name: Rc<str>,
  map: HashMap<Rc<str>, Value>
}

impl Module {
  pub fn new(name: Rc<str>) -> Self {
    Module {
      name,
      map: HashMap::new()
    }
  }
  pub fn set(&mut self, key: Rc<str>, value: Value) {
    if self.map.try_insert(key.clone(), value).is_err() {
      ref_error!("{key:?} is already defined in this scope");
    }
  }
}
pub enum Value {
  Module(Module)
}
