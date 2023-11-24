use crate::{
  types::{ StringPtr, StructPropertyMap, StructPtr },
  values::Value
};

#[derive(Debug)]
pub struct Struct {
  pub name: StringPtr,
  // sorted map, fast search
  pub props: StructPropertyMap
}

impl PartialEq for Struct {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    // each struct has its own name, even if they have the same name
    StringPtr::ptr_eq(&self.name, &other.name)
  }
}

#[derive(Debug)]
pub struct StructProperty {
  pub prototype: StructPtr,
  pub default_value: Option<Value>
}