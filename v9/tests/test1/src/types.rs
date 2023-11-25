use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap
};
use hashbrown::HashMap;

#[allow(unused_imports)]
use crate::{
  enums::*,
  instances::*,
  structs::*,
  tests::*,
  values::*,
  variants::*,
};

pub type SlicePtr<T> = Box<[T]>;
pub type Pointer<T> = Rc<T>;
pub type Map<T> = HashMap<StringPtr, T>;
pub type StructPtr = Pointer<Struct>;
pub type StringPtr = Pointer<str>;
pub type StructPropertyMap = BinTreeMap<StringPtr, StructProperty>;
pub type VariantMap = HashMap<usize, StructPtr>;
pub type EnumPtr = Pointer<Enum>;
