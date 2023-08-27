use {
  std::collections::HashMap,
  crate::{
    aliases::Id,
    functions::Function,
    namespaces::Namespace
  }
};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  None,
  String(String),
  Id(Id),
  Vec(Vec<Value>),
  Dict(HashMap<Id, Value>),
  Function(Box<Function>),
  Namespace(Box<Namespace>),
  // Struct(Box<Struct>),
  // Instance(Box<Instance>),
}

impl Value {
  pub fn typename(&self) -> &str {
    use Value::*;
    match self {
      None => "none",
      String(_) | Id(_) => "string",
      Vec(_) => "vector",
      Dict(_) => "dictionary",
      Function(_) => "function",
      Namespace(_) => "namespace",
      // Struct(_) => "struct"
      // Instance(obj) => format!("instance of {}", obj.parent).as_str()
    }
  }
}