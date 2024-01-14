use std::rc::Rc;
use hashbrown::HashMap;
use crate::context::Context;

pub fn parse_module_with_name(ctx: &mut Context, name: Rc<str>) -> (Rc<str>, Value) {
  let module = Module::new(name);
  while ctx.next_char() != '}' {
    let word = ctx.expect_word();
    let (name, value) = match word.as_str() {
      "mod" => parse_module(&mut ctx),
      | "class"
      | "struct"
      | "enum"
      | "flagset"
      | "use"
      | "mixin"
      | "impl"
      | "const"
      | "test"
        => todo!("{word:?} not implemented yet"),
      _ => syn_error!("unexpected word {word:?} here")
    };
    module.set(name, value);
  }
  Value::Module(module)
}

pub fn parse_module(ctx: &mut Context) -> (Rc<str>, Value) {
  let name: Rc<str> = Rc::from(ctx.expect_word());
  ctx.expect_char('{');
  let module = parse_module_with_name(ctx, name.clone());
  (name, module)
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
    if let Err(_) = self.map.try_insert(key, value) {
      ref_error!("{key:?} is already defined in this scope");
    }
  }
}
