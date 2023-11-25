use crate::{
  types::{StructPtr, StringPtr},
  instances::Instance
};

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Value {
  // None,
  String(StringPtr),
  Struct(StructPtr),
  Instance(Instance),
  Uint8(u8)
}

impl Value {
  // pub fn is_same_variant(&self, other: &Self) -> bool {
  //   discriminant(self) == discriminant(other)
  // }
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::new();
    match self {
      Self::Instance(instance) => string.push_str(&instance.debug(depth + 1)),
      Self::Struct(structure) => string.push_str(&structure.debug(depth + 1)),
      Self::Uint8(n) => string.push_str(&format!("{n}_u8")),
      _ => unimplemented!()
    }
    string
  }
}