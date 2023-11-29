use crate::{
  types::{ ClassPtr, StringPtr, SlicePtr, Map },
  instances::Instance, functions::FunctionPtr
};

#[allow(unused)]
#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Value {
  None,
  UnsignedInt(u64), // uint
  Int(i64), // int
  Float(f64), // float
  Usize(usize), // usize
  Isize(isize), // isize
  Vec(SlicePtr<Value>), /// Rc<Mutex<?>> | Vec<Value>
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
      Self::UnsignedInt(n) => string.push_str(&format!("{n}u")),
      _ => unimplemented!()
    }
    string
  }
}