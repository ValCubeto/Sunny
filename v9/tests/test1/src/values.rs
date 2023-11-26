use crate::{
  types::{ ClassPtr, StringPtr, SlicePtr, Map },
  instances::Instance, functions::FunctionPtr
};

#[allow(unused)]
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Value {
  None,
  Uint8(u8),
  Uint16(u16),
  Uint32(u32),
  Uint64(u64),
  Uint128(u128),
  Usize(usize),
  Int8(i8),
  Int16(i16),
  Int32(i32),
  Int64(i64),
  Int128(i128),
  Isize(isize),
  Float32(f32),
  Float64(f64),
  Vec(SlicePtr<Value>), // Rc<Mutex<?>> | Vec<Value>
  String(StringPtr),
  Dict(Map<Value>),
  Class(ClassPtr),
  Instance(Instance),
  Function(FunctionPtr)
}

impl Value {
  // pub fn is_same_variant(&self, other: &Self) -> bool {
  //   discriminant(self) == discriminant(other)
  // }
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::new();
    match self {
      Self::Instance(instance) => string.push_str(&instance.debug(depth + 1)),
      Self::Class(class) => string.push_str(&class.debug(depth + 1)),
      Self::Uint8(n) => string.push_str(&format!("{n}_u8")),
      _ => unimplemented!()
    }
    string
  }
}