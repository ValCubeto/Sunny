use std::{rc::Rc, sync::Arc};

fn main() {
  let num = 99_u8;
  let string = "hello";
  let d: Box<str> = Box::from(string);
  let values: Vec<ValuePtr> = vec![
    Box::new(num),
    d.into()
  ];
}
