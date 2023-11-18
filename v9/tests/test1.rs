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
pub type Properties = BinTreeMap<Id, StructRef>;
pub type Parameters<'a> = BinTreeMap<Id, Value>;
pub type InstanceValues<'a> = &'a [Value];

pub enum Value {
  String(Id),
  Uint8(u8)
}

pub struct Struct {
  pub name: Id,
  pub properties: Properties
}

impl<'instance> Struct {
  pub fn match_params(&self, values: Parameters) -> Vec<Value> {
    vec![]
  }
}

pub struct Instance<'instance> {
  pub structure: StructRef,
  pub values: InstanceValues<'instance>
}

impl<'instance> Instance<'instance> {
  // checks if the properties are correct
  pub fn new(structure: StructRef, params: Parameters<'instance>) -> Self {
    Self {
      values: structure.match_params(params),
      structure,
    }
  }
}

pub struct Mod<'instance> {
  name: Id,
  values: Map<Instance<'instance>>
}

fn main() {
  let u8_properties = Properties::new();
  let u8_struct = Rc::new(Struct {
    name: "u8".into(),
    properties: u8_properties
  });

  let point_properties = Properties::from([
    ("x".into(), Rc::clone(&u8_struct)),
    ("y".into(), Rc::clone(&u8_struct)),
  ]);
  let point_struct = Rc::new(Struct {
    name: "Point".into(),
    properties: point_properties
  });

  let values = Parameters::from([
    ("x".into(), Value::Uint8(32)),
    ("y".into(), Value::Uint8(5))
  ]);
  let my_point = Instance::new(
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