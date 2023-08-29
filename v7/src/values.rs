use std::rc::Rc;

use crate::aliases::Dict;

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
  Dict(Rc<Dict>),
  Function(Rc<Function>),
  Namespace(Rc<Namespace>),
  Struct(Rc<Struct>),
  Instance(Rc<Instance>),
}