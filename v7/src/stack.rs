use crate::{
  aliases::{ Id, Dict },
  values::Value,
  reference_error
};

#[derive(Debug)]
pub struct Stack(Vec<Dict>);

impl Stack {
  pub fn new() -> Self {
    Stack(Vec::with_capacity(2))
  }
  pub fn get_value(&mut self, id: &Id) -> &Value {
    for space in &self.0 {
      if let Some(value) = space.get(id) {
        return value;
      }
    }
    reference_error!("{id:?} is not defined");
  }
  pub fn assign(&mut self, id: Id, value: Value) {
    let last = self.0.last_mut().unwrap();
    // if !last.contains_key(&id) { reference_error!("{id:?} is not defined"; ctx); }
    // if typeof(last.get(&id)) != typeof(value) { type_error!("cannot assign a {t1:?} to a {t2:?}"; ctx) }
    last.insert(id, value);
  }
  pub fn declare(&mut self, id: Id, value: Value) {
    let last = self.0.last_mut().unwrap();
    if last.contains_key(&id) {
      reference_error!("{id:?} is already defined");
    }
    last.insert(id, value);
  }
  pub fn preppend(&mut self, value: Dict) {
    self.0.insert(0, value);
  }
}