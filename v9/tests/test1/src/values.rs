use crate::{
  types::StructPtr,
  instances::Instance
};

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Value {
  // None,
  Struct(StructPtr),
  Instance(Instance),
  Uint8(u8)
}

// impl Value {
//   pub fn is_same_variant(&self, other: &Self) -> bool {
//     discriminant(self) == discriminant(other)
//   }
// }