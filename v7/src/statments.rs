use crate::{
  aliases::{ Id, Arguments },
  expressions::Expression
};

// FIXME: change { id: Id } to { path: Vec<Id> }
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statment {
  Declaration {
    id: Id,
    mutable: bool,
    expr: Expression
  },
  Assignment {
    id: Id,
    expr: Expression
  },
  Call {
    id: Id,
    args: Arguments
  }
}