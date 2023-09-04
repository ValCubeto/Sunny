use {
  std::collections::HashMap,
  crate::{
    aliases::Id,
    values::Value,
    functions::{ Function, FunctionValue },
    { hashmap, builtin_function }
  }
};

pub fn make_global() -> HashMap<Id, Value> {
  hashmap! {
    println => builtin_function!("println", |args| {
      println!("{:?}", args[0]);
      Value::Null
    }),
    print => builtin_function!("print", |args| {
      print!("{:?}", args[0]);
      Value::Null
    }),
    null => Value::Null
  }
}