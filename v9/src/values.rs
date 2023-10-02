use crate::aliases::{ Dict, Array, /* Id, ConstArray */ };

#[allow(non_camel_case_types, unused)]
#[derive(Debug)]
pub enum Value {
  i8(i8),
  u8(u8),
  isize(isize),
  usize(usize),
  Dict(Dict),
  Array(Array), // ArrayValue
  String(String), // StringValue
}

// pub enum StringValue {
//   Mutable(String),
//   Constant(Id)
// }

// pub enum ArrayValue {
//   Mutable(Vec<Value>),
//   Constant(ConstArray<Value>)
// }