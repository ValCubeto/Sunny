mod types;
mod values;
mod structs;
mod instances;
mod enums;

use types::*;
use values::*;
use structs::*;
use instances::*;
use enums::*;

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
        prototype: StructPtr::clone(&u8_struct),
        default_value: None
      }),
      ("y".into(), StructProperty {
        prototype: StructPtr::clone(&u8_struct),
        default_value: None
      }),
    ])
  });

  // let point = Point { y: 5, x: 10 } (order does not matter)
  let point_instance = point_struct.new_instance(Map::from([
    ("y".into(), Instance {
      prototype: StructPtr::clone(&u8_struct),
      props: Box::new([ Value::Uint8(5) ])
    }),
    ("x".into(), Instance {
      prototype: StructPtr::clone(&u8_struct),
      props: Box::new([ Value::Uint8(10) ])
    }),
  ]));

  let mut stack = Map::from([
    ("Point".into(), Value::Struct(StructPtr::clone(&point_struct))),
    ("point".into(), Value::Instance(point_instance))
  ]);

  dbg!(point_struct == u8_struct);
  match stack.get_mut("point").unwrap() {
    Value::Instance(point) => {
      dbg!(&point.prototype.name);
      dbg!(point.get_property("x".into()));
      // point.x = 20
      point.set_property("x".into(), Value::Instance(Instance {
        prototype: StructPtr::clone(&u8_struct),
        props: Box::new([ Value::Uint8(20) ])
      }));
      dbg!(point.get_property("x".into()));
    }
    _ => unreachable!()
  };
}
