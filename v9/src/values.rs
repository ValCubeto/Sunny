use crate::aliases::{ DictPtr, ArrayPtr, Id };

#[allow(non_camel_case_types, unused)]
#[derive(Debug)]
pub enum Value {
  Dict(DictPtr),
  Array(ArrayPtr),
  String(Id),
  i8(i8),
  u8(u8),
  isize(isize),
  usize(usize),
  Function(fn ())
}
