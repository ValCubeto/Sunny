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
  values::Value,
  aliases::Id,
};

pub fn main() {
  error!("SyntaxError"; "test");
  let jhonn_id = Id::from("Jhonn");
  let marie_id = Id::from("Marie");
  let values = dict! {
    Id::clone(&jhonn_id) => Value::u8(30),
    Id::clone(&marie_id) => Value::String("treinta".into())
  };

  debug!(size_of::<Value>());
  println!();
  debug!(values);
}
