use crate::reference_error;

use {
  std::collections::HashMap,
  crate::{
    aliases::Id,
    values::Value
  }
};

pub trait Stack {
  fn get_value(&self, id: &Id) -> &Value;
  fn set_value(&mut self, id: Id, value: Value);
}

impl Stack for Vec<HashMap<Id, Value>> {
  fn get_value(&self, id: &Id) -> &Value {
    for space in self {
      let value = space.get(id);
      if let Some(value) = value {
        return value
      }
    }
    reference_error!("hola")
  }
  fn set_value(&mut self, id: Id, value: Value) {
    self
      .last_mut()
      .unwrap()
      .insert(id, value);
  }
}