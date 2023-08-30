use {
  std::rc::Rc,
  crate::{
    aliases::Id,
    values::Value
  }
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Struct {
  pub name: Id,
  pub props: Vec<(Id, Rc<Struct>, Value)>
}