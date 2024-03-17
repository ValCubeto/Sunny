use std::rc::Rc;

pub enum Value {
  String(Rc<str>),
  Uint(u8),
  Int(i8),
}
