use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap
};
use hashbrown::HashMap;
use crate::structs::{ Struct, StructProperty };

pub type SlicePtr<T> = Box<[T]>;
pub type Pointer<T> = Rc<T>;
pub type Map<T> = HashMap<StringPtr, T>;
pub type StructPtr = Pointer<Struct>;
pub type StringPtr = Pointer<str>;
pub type StructPropertyMap = BinTreeMap<StringPtr, StructProperty>;
pub type VariantMap = HashMap<usize, StructPtr>;

