extern crate hashbrown;
use std::rc::Rc;

pub type Id = Rc<str>;
pub type Map<T> = hashbrown::HashMap<Id, T>;
pub type StructRef<'a> = Rc<Struct<'a>>;
pub type Properties<'a> = &'a [Property<'a>];

pub struct Property<'a> {
  name: Id,
  prototype: StructRef<'a>
}

pub enum Value {
  String(Id),
  Uint8(u8)
}

pub struct Struct<'a> {
  name: Id,
  properties: Properties<'a>
}

pub struct Instance<'a, 'b> {
  prototype: StructRef<'a>,
  values: &'b [Value]
}

pub struct Mod<'a, 'b> {
  name: Id,
  values: Map<Instance<'a, 'b>>
}

fn main() {
  let u8_properties = &vec![];
  let u8_struct = Rc::new(Struct {
    name: "u8".into(),
    properties: u8_properties
  });

  let point_properties = &vec![
    Property {
      name: "x".into(),
      prototype: Rc::clone(&u8_struct)
    },
    Property {
      name: "y".into(),
      prototype: Rc::clone(&u8_struct)
    },
  ];
  let point_struct = Rc::new(Struct {
    name: "Point".into(),
    properties: point_properties
  });

  let values = &vec![
    Value::Uint8(32),
    Value::Uint8(5),
  ];
  let my_point = Instance {
    prototype: Rc::clone(&point_struct),
    values
  };

  let module = Mod {
    name: "main".into(),
    values: Map::from([
      ("point".into(), my_point)
    ])
  };
}