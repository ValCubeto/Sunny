use crate::aliases::{ Dict, Array, /* Id, ConstArray */ };

#[allow(non_camel_case_types, unused)]
#[derive(Debug)]
pub enum Value {
  u8(u8),
  String(String), // StringValue
  Array(Array), // ArrayValue
  Dict(Dict)
}

// pub enum StringValue {
//   Mutable(String),
//   Constant(Id)
// }

// pub enum ArrayValue {
//   Mutable(Vec<Value>),
//   Constant(ConstArray<Value>)
// }