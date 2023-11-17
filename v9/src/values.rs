use crate::aliases::{ DictPtr, ArrayPtr, Id, FunctionPtr };

#[allow(non_camel_case_types, unused)]
#[derive(Clone)]
pub enum Value {
  i8(i8),
  u8(u8),
  isize(isize),
  usize(usize),
  Dict(DictPtr),
  Array(ArrayPtr),
  String(Id),
  Function(FunctionPtr)
}
