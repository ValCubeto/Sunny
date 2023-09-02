
use {
  std::rc::Rc,
  crate::{
    aliases::Dict,
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
  Null,
  String(String),
  Id(Id),
  Vec(Vec<Value>),
  // Array(Box<[Value]>),
  Dict(Rc<Dict>),
  Function(Rc<Function>),
  Namespace(Rc<Namespace>),
  Struct(Rc<Struct>),
  Instance(Rc<Instance>),
}