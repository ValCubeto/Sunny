use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap,
  any::Any
};

use hashbrown::HashMap;

#[derive(Clone)]
enum Value {
  None,
  Struct(StructPtr),
  Instance(Instance),
  Uint8(u8)
}
type Map<T> = HashMap<StringPtr, T>;
type StructPtr = Rc<Struct>;
type StringPtr = Rc<str>;
type StructPropertyMap = BinTreeMap<StringPtr, StructProperty>;

struct Struct {
  name: StringPtr,
  // sorted map, fast search
  props: StructPropertyMap
}

struct StructProperty {
  structure: StructPtr,
  default_value: Option<Value>
}

#[derive(Clone)]
struct Property {
  structure: StructPtr,
  value: Value
}

#[derive(Clone)]
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

  // where to store the number??????????? this is not valid
  let _u8_instance = Instance {
    structure: StructPtr::clone(&u8_struct),
    props: Vec::new()
  };

  let u8_value = Value::Uint8(5);

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
  dbg!(stack.type_id());
}