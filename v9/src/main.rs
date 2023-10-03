mod colors;
mod about;
mod macros;
mod aliases;
mod values;
mod context;
mod args;
mod run;
mod repl;

use std::mem::size_of;
use crate::{
  values::Value, aliases::{Dict, Id},
  // aliases::{ Id, Dict, Array }
};

pub fn main() {
  let jhonn_id = Id::from("Jhonn");
  let marie_id = Id::from("Marie");
  let _values = Value::Dict(Dict::new(hashmap! {
    Id::clone(&jhonn_id) => Value::u8(30),
    Id::clone(&marie_id) => Value::String("treinta".into())
  }));

  debug!(size_of::<Value>());
}
