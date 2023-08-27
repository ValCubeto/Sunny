use crate::aliases::Dict;

use {
  std::collections::HashMap,
  crate::{
    context::Context,
    aliases::Id,
    values::Value,
    functions::parse_function,
    { syntax_error, reference_error }
  }
};

pub fn parse_namespace(ctx: &mut Context, name: Id) -> Namespace {
  let mut namespace = Namespace::new(name);

  ctx.go();
  if ctx.current != '{' {
    syntax_error!("expected '{{', found {:?}", ctx.current; ctx);
  }
  
  ctx.next_char();
  ctx.go();

  while ctx.current != '}' {
    let word: String = ctx.expect_word();
    match word.as_str() {
      "namespace" => {
        ctx.go();
        let name = Id::from(ctx.expect_word());
        let value = parse_namespace(ctx, name.clone());
        namespace.set(name, Value::Namespace(Box::new(value)));
      }
      "fun" => {
        ctx.go();
        let name = Id::from(ctx.expect_word());
        let value = parse_function(ctx, name.clone());
        namespace.set(name, Value::Function(Box::new(value)));
      }
      "async" | "struct" | "extend" | "const" | "import" => syntax_error!("{word:?} not implemented"; ctx),
      _ => syntax_error!("unexpected identifier {word:?} here"; ctx)
    }
    ctx.next_char();
    ctx.go();
  }

  namespace
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace {
  pub name: Id,
  pub public: Dict,
  pub private: Dict
}

impl Namespace {
  pub fn new(name: Id) -> Self {
    Namespace {
      name,
      public: HashMap::new(),
      private: HashMap::new(),
    }
  }
  #[allow(unused)]
  pub fn get(&self, id: &Id) -> Option<&Value> {
    self.public.get(id)
  }
  pub fn set(&mut self, id: Id, value: Value) {
    if self.public.contains_key(&id) {
      reference_error!("identifier {id:?} already used");
    }
    // check same type
    self.public.insert(id, value);
  }
}