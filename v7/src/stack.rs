use crate::{
  aliases::{ Id, Dict },
  values::Value,
  reference_error
};

pub trait Stack {
  fn get_value(&self, id: &Id) -> &Value;
  fn set_value(&mut self, id: Id, value: Value);
  fn preppend(&mut self, value: Dict);
}

impl Stack for Vec<Dict> {
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
  fn preppend(&mut self, value: Dict) {
    self.insert(0, value);
  }
  fn expect<T>(self, i: u8) -> T {
    i as Value;
  }
}