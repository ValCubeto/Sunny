use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap,
  any::type_name
};

use hashbrown::HashMap;

#[derive(Clone, Debug)]
enum Value {
  _None,
  Struct(StructPtr),
  Instance(Instance),
  Uint8(u8)
}
type Map<T> = HashMap<StringPtr, T>;
type StructPtr = Rc<Struct>;
type StringPtr = Rc<str>;
type StructPropertyMap = BinTreeMap<StringPtr, StructProperty>;

#[derive(Debug)]
struct Struct {
  name: StringPtr,
  // sorted map, fast search
  props: StructPropertyMap
}

#[derive(Debug)]
struct StructProperty {
  structure: StructPtr,
  default_value: Option<Value>
}

#[derive(Clone)]
struct Property {
  structure: StructPtr,
  value: Value
}

#[derive(Clone, Debug)]
struct Instance {
  structure: StructPtr,
  props: Vec<Value>
}

trait CreateInstance {
  fn new_instance(&self, props: Map<Value>) -> Instance;
}
impl CreateInstance for StructPtr {
  fn new_instance(&self, candidates: Map<Value>) -> Instance {
    let mut props = Vec::with_capacity(self.props.len());
    for (key, prop) in self.props.iter() {
      match candidates.get(key) {
        Some(value) => {
          props.push(value.clone());
        },
        None => {
          match &(prop.default_value) {
            Some(default_value) => {
              props.push(default_value.clone());
            }
            None => panic!("missing field {key:?}")
          }
        }
      }
    }
    Instance {
      structure: StructPtr::clone(self),
      props
    }
  }
}

fn main() {
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

  let point_instance = point_struct.new_instance(Map::from([
    ("y".into(), Value::Uint8(5)),
    ("x".into(), Value::Uint8(10)),
  ]));

  let stack = Map::from([
    ("Point".into(), Value::Struct(point_struct)),
    ("point".into(), Value::Instance(point_instance))
  ]);
  dbg!(type_of(&stack));
  dbg!(&stack["Point"]);
  dbg!(&stack["point"]);
  match &stack["point"] {
    Value::Instance(point) => {
      dbg!(&point.structure.name);
    }
    _ => unreachable!()
  }
}

fn type_of<T>(_value: &T) -> &str {
  type_name::<T>()
}