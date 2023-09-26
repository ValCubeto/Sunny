use {
  std::collections::HashMap,
  crate::{
    aliases::Id,
    values::Value,
    { hashmap, builtin_function }
  }
};

pub fn make_global() -> HashMap<Id, Value> {
  #[allow(non_snake_case)]
  let (PRINTLN, PRINT, READLN) = (Id::from("println"), Id::from("print"), Id::from("readln"));
  hashmap! {
    (Id::clone(&PRINTLN)) => builtin_function!(PRINTLN, |args| {
      println!("{:?}", args[0]);
      Value::Null
    }),
    (Id::clone(&PRINT)) => builtin_function!(PRINT, |args| {
      print!("{:?}", args[0]);
      Value::Null
    }),
    (Id::clone(&READLN)) => builtin_function!(READLN, |_args| {
      let mut buf = String::new();
      std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read the line");
      Value::String(buf.trim().to_owned())
    }),
    (Id::from("null")) => Value::Null,
  }
}