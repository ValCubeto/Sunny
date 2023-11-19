use std::{
  rc::Rc,
  collections::BTreeMap as BinTreeMap
};

use hashbrown::HashMap;

enum Value {
  None,
  Uint8(u8)
}
type Map<T> = HashMap<Id, T>;
type StructRef = Rc<Struct>;
type Id = Rc<str>;
type PropertyMap = BinTreeMap<Id, Property>;

struct Struct {
  name: Id,
  // sorted map, fast search
  props: PropertyMap
}

struct Property {
  structure: StructRef,
  value: Value
}

struct Instance {
  structure: StructRef,
  props: Vec<Property>
}

trait CreateInstance {
  fn new_instance(&self, props: &Map<Property>) -> Instance;
}
impl CreateInstance for StructRef {
  fn new_instance(&self, props: &Map<Property>) -> Instance {
    let mut props = Vec::with_capacity(self.props.len());
    props.push(Property { structure: StructRef::clone(self), value: Value::Uint8(8) });
    Instance {
      structure: StructRef::clone(self),
      props
    }
  }
}

enum InstanceValue {
  Props,
  Value
}
type Stack = Map<>;

fn main() {
  let u8_struct = StructRef::new(Struct {
    name: "u8".into(),
    props: PropertyMap::new()
  });

  // where to store the number???????????
  let u8_instance = Instance {
    structure: StructRef::clone(&u8_struct),
    props: <Vec<Property>>::new()
  };

  // struct Point { x: u8, y: u8 }
  let point_struct = Rc::new(Struct {
    name: "Point".into(),
    props: PropertyMap::from([
      ("x".into(), Property {
        structure: StructRef::clone(&u8_struct),
        value: Value::None
      }),
      ("y".into(), Property {
        structure: StructRef::clone(&u8_struct),
        value: Value::None
      }),
    ])
  });

  let props = Map::from([]);
  let instance = point_struct.new_instance(&props);
}