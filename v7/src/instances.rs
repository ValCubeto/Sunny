use {
  std::rc::Rc,
  crate::{
    structs::Struct,
    values::Value
  }
};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
  pub parent: Rc<Struct>,
  pub values: Box<[Value]>
}