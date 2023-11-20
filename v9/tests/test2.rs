use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap
};

use hashbrown::HashMap;

enum Value {
  None,
  Struct(StructPtr),
  Instance(Instance),
  Uint8(u8)
}
type Map<T> = HashMap<StringPtr, T>;
type StructPtr = Rc<Struct>;
type StringPtr = Rc<str>;
type PropertyMap = BinTreeMap<StringPtr, Property>;

struct Struct {
  name: StringPtr,
  // sorted map, fast search
  props: PropertyMap
}

struct Property {
  structure: StructPtr,
  value: Value
}

struct Instance {
  structure: StructPtr,
  props: Vec<Property>
}

trait CreateInstance {
  fn new_instance(&self, props: Map<Property>) -> Instance;
}
impl CreateInstance for StructPtr {
  fn new_instance(&self, props: Map<Property>) -> Instance {
    let mut props = Vec::with_capacity(self.props.len());
    for (key, value) in self.props.iter() {
      value.value;
    }
    props.push(Property { structure: StructPtr::clone(self), value: Value::Uint8(8) });
    Instance {
      structure: StructPtr::clone(self),
      props
    }
  }
}

fn main() {
  let u8_struct = StructPtr::new(Struct {
    name: "u8".into(),
    props: PropertyMap::new()
  });

  // where to store the number???????????
  let u8_instance = Instance {
    structure: StructPtr::clone(&u8_struct),
    props: Vec::new()
  };

  // struct Point { x: u8, y: u8 }
  let point_struct = StructPtr::new(Struct {
    name: "Point".into(),
    props: PropertyMap::from([
      ("x".into(), Property {
        structure: StructPtr::clone(&u8_struct),
        value: Value::None
      }),
      ("y".into(), Property {
        structure: StructPtr::clone(&u8_struct),
        value: Value::None
      }),
    ])
  });

  let point_instance = point_struct.new_instance(Map::from([]));

  let stack = Map::from([
    ("Point".into(), Value::Struct(point_struct)),
    ("point".into(), Value::Instance(point_instance))
  ]);
}