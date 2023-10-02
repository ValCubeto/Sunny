use std::rc::Rc;

use crate::aliases::{ Dict, Array, Id, /* Id, ConstArray */ };

#[allow(non_camel_case_types, unused)]
#[derive(Debug)]
pub enum Value {
  Dict(Dict),
  Array(Array),
  String(Id),
  i8(i8),
  u8(u8),
  isize(isize),
  usize(usize),
  Function(fn ())
}

// pub enum StringValue {
//   Mutable(String),
//   Constant(Id)
// }

// pub enum ArrayValue {
//   Mutable(Vec<Value>),
//   Constant(ConstArray<Value>)
// }