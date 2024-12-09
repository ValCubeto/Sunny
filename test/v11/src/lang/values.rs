use super::types::BuiltInType;
// use crate::parse::Number;

pub enum Value {
  BuiltInType(BuiltInType),

  Boolean(bool),

  UInt8(u8),
  UInt16(u16),
  UInt32(u32),
  UInt64(u64),
  USize(usize),

  Int8(i8),
  Int16(i16),
  Int32(i32),
  Int64(i64),
  ISize(isize),

  Float32(f32),
  Float64(f64),

  Tuple(*const [Value]),
  // Vec(Vector),
  // Vec(*mut ()),
  // Str(*const str),
  String(String),
  Char(char),

  // Function(Function),

  // HashMap(HashMap<*const str, Value>)
}

#[derive(Debug)]
pub enum IntermediateValue {
  // Number(Number),
  Ident(String)
}

// #[allow(unused)]
// pub struct Vector {
//   /// Elements size in bytes
//   elem_size: u8,
//   /// Pointer size in bytes
//   /// Usually 32 bits long because of relative pointers
//   ptr_size: u8,
//   // HINT: if relative pointers are used, I should reduce this size
//   ptr: usize,
//   /// Will use the size of ptr_size
//   len: usize
// }
