mod macros;
mod aliases;
mod values;
mod args;
mod run;
mod repl;

use std::mem::size_of;
use crate::{
  values::Value,
  aliases::{ Id, Dict, Array }
};

pub fn main() {
  // let jhonn_id = Id::from("Jhonn");
  // let marie_id = Id::from("Marie");
  // let values = Value::Dict(Dict::new(hashmap! {
  //   Id::clone(&jhonn_id) => Value::u8(30),
  //   Id::clone(&marie_id) => Value::String("".to_string())
  // }));

  println!("Size of Value: {} bytes", size_of::<Value>());
  println!();
  println!("Size of u8: {} bytes", size_of::<u8>());
  println!("Size of usize: {} bytes", size_of::<usize>());
  println!("Size of u128: {} bytes", size_of::<u128>());
  println!();
  println!("Size of String: {} bytes", size_of::<String>());
  println!("Size of Id: {} bytes", size_of::<Id>());
  println!();
  println!("Size of Array: {} bytes", size_of::<Array>());
  println!();
  println!("Size of Dict: {} bytes", size_of::<Dict>());
  // println!();
  // println!("values = {values:?}");
}
