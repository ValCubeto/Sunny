use {
  std::{
    collections::HashMap,
    rc::Rc
  },
  crate::{
    structs::Struct,
    values::Value,
    aliases::Id
  }
};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
  pub parent: Rc<Struct>,
  pub values: HashMap<Id, Value>
}