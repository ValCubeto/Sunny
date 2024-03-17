use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap
};
use hashbrown::HashMap;

#[allow(unused_imports)]
use crate::{
  enums::*,
  instances::*,
  classes::*,
  tests::*,
  values::*,
  variants::*,
};

pub type SlicePtr<T> = Box<[T]>;
pub type Pointer<T> = Rc<T>;
pub type Map<T> = HashMap<StringPtr, T>;
pub type ClassPtr = Pointer<Class>;
pub type StringPtr = Pointer<str>;
pub type ClassPropertyMap = BinTreeMap<StringPtr, ClassProperty>;
pub type VariantMap = HashMap<usize, ClassPtr>;
pub type EnumPtr = Pointer<Enum>;
