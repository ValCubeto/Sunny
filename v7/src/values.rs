use {
  std::collections::HashMap,
  crate::{
    aliases::Id,
    functions::Function,
    namespaces::Namespace,
    structs::Struct,
    instances::Instance
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
  Struct(Box<Struct>),
  Instance(Box<Instance>),
}