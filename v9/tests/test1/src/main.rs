use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap
};

use hashbrown::HashMap;

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

pub type Slice<T> = Box<[T]>;
pub type Pointer<T> = Rc<T>;
pub type Map<T> = HashMap<StringPtr, T>;
pub type StructPtr = Pointer<Struct>;
pub type StringPtr = Pointer<str>;
pub type StructPropertyMap = BinTreeMap<StringPtr, StructProperty>;

#[derive(Debug)]
pub struct Struct {
  name: StringPtr,
  // sorted map, fast search
  props: StructPropertyMap
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
  structure: StructPtr,
  default_value: Option<Value>
}

#[derive(Clone, Debug)]
pub struct Instance {
  structure: StructPtr,
  props: Slice<Value>
}

impl Instance {
  // TODO: i should cache
  pub fn get_property(&self, key: StringPtr) -> Value {
    let search = self.structure.props.iter().position(|(curr_key, _)| *curr_key == key);
    let index = match search {
      Some(index) => index,
      None => panic!("'{}' has no property '{}'", self.structure.name, key),
    };
    self.props[index].clone()
  }
  pub fn try_get_property(&self, key: StringPtr) -> Option<Value> {
    let search = self.structure.props.iter().position(|(curr_key, _)| *curr_key == key);
    let index = match search {
      Some(index) => index,
      None => return None
    };
    Some(self.props[index].clone())
  }
}

pub trait CreateInstance {
  fn new_instance(&self, props: Map<Instance>) -> Instance;
}
impl CreateInstance for StructPtr {
  fn new_instance(&self, mut candidates: Map<Instance>) -> Instance {
    let mut props = Vec::with_capacity(self.props.len());
    for (key, prop) in self.props.iter() {
      // this does not deallocate
      match candidates.remove(key) {
        Some(instance) => {
          if instance.structure != prop.structure {
            panic!("Mismatched types. '{}.{}' has type '{}', found '{}'.", self.name, key, prop.structure.name, instance.structure.name);
          };
          props.push(Value::Instance(instance));
        },
        None => {
          match &(prop.default_value) {
            Some(default_value) => {
              props.push(default_value.clone());
            }
            None => panic!("Missing property '{}' to create a '{}'", key, self.name)
          }
        }
      }
    }
    if !candidates.is_empty() {
      if candidates.len() == 1 {
        let key = candidates.keys().next().unwrap();
        panic!("'{}' has no property '{}'", self.name, key);
      }
      let keys = candidates.keys()
        // .take(3)
        .cloned()
        .collect::<Vec<_>>()
        .join("', '");
      panic!("'{}' has no properties '{}'", self.name, keys);
    }
    Instance {
      structure: StructPtr::clone(self),
      props: props.into()
    }
  }
}

fn main() {
  // class u8 { struct { val: _ } }
  let u8_struct = StructPtr::new(Struct {
    name: "u8".into(),
    props: StructPropertyMap::new()
  });

  // struct Point { x: u8, y: u8 }
  let point_struct = StructPtr::new(Struct {
    name: "Point".into(),
    props: StructPropertyMap::from([
      ("x".into(), StructProperty {
        structure: StructPtr::clone(&u8_struct),
        default_value: None
      }),
      ("y".into(), StructProperty {
        structure: StructPtr::clone(&u8_struct),
        default_value: None
      }),
    ])
  });

  // let point = Point { y: 5, x: 10 } (order does not matter)
  let point_instance = point_struct.new_instance(Map::from([
    ("y".into(), Instance {
      structure: StructPtr::clone(&u8_struct),
      props: Box::new([ Value::Uint8(5) ])
    }),
    ("x".into(), Instance {
      structure: StructPtr::clone(&u8_struct),
      props: Box::new([ Value::Uint8(10) ])
    }),
  ]));

  let stack = Map::from([
    ("Point".into(), Value::Struct(point_struct)),
    ("point".into(), Value::Instance(point_instance))
  ]);

  dbg!(&stack["Point"]);
  dbg!(&stack["point"]);
  match &stack["point"] {
    Value::Instance(point) => {
      dbg!(&point.structure.name);
      dbg!(point.get_property("x".into()));
    }
    _ => unreachable!()
  };
}
