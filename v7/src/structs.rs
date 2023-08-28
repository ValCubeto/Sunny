use {
  std::{
    collections::HashMap,
    rc::Rc
  },
  crate::{
    aliases::Id,
    values::Value
  }
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Struct {
  pub extended: Option<Vec<Rc<Struct>>>,
  pub name: Id,
  pub props: HashMap<Id, (Option<Rc<Struct>>, Option<Value>)>
}