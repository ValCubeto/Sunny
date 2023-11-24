use crate::{
  types::{ Pointer, VariantMap, StringPtr },
  instances::Instance
};

#[derive(Debug)]
pub struct Enum {
  pub name: StringPtr,
  pub variants: VariantMap
}

pub type EnumPtr = Pointer<Enum>;

#[derive(Debug)]
pub struct Variant {
  pub prototype: EnumPtr,
  pub variant_id: usize,
  pub value: Instance
}
