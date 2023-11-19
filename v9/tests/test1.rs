extern crate hashbrown;
use std::{
  rc::Rc,
  sync::Mutex,
  collections::BTreeMap as BinTreeMap
};

pub type RefMut<T> = Rc<Mutex<T>>;
pub type Id = Rc<str>;
pub type Map<T> = hashbrown::HashMap<Id, T>;

pub type StructRef = Rc<Struct>;
pub type StructProperties = BinTreeMap<Id, StructRef>;
// can be invalid and turned into InstanceProperties
pub type PropertyCandidates = BinTreeMap<Id, Value>;
pub type InstanceProperties = hashbrown::HashSet<Value>;

pub enum Value {
  String(Id),
  Uint8(u8)
}

pub struct Struct {
  pub name: Id,
  pub properties: StructProperties
}

impl Struct {
  pub fn new_instance(&self, values: PropertyCandidates) -> Vec<Value> {
    vec![]
  }
}

pub struct Instance {
  pub structure: StructRef,
  pub values: InstanceProperties
}

pub struct Mod<'a> {
  name: Id,
  values: Map<Instance<'a>>
}

fn main() {
  let u8_properties = StructProperties::new();
  let u8_struct = Rc::new(Struct {
    name: "u8".into(),
    properties: u8_properties
  });

  let point_properties = StructProperties::from([
    ("x".into(), Rc::clone(&u8_struct)),
    ("y".into(), Rc::clone(&u8_struct)),
  ]);
  let point_struct = Rc::new(Struct {
    name: "Point".into(),
    properties: point_properties
  });

  let values = PropertyCandidates::from([
    ("x".into(), Value::Uint8(32)),
    ("y".into(), Value::Uint8(5))
  ]);
  let my_point = Struct::new_instance(
    Rc::clone(&point_struct),
    values
  );

  let module = Mod {
    name: "main".into(),
    values: Map::from([
      ("point".into(), my_point)
    ])
  };
}