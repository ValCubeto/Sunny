use crate::{
  types::*,
  values::*,
  structs::*,
  instances::*,
  enums::*
};

pub fn test_structs() {
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

pub fn test_enums() {
  let u8_struct = StructPtr::new(Struct {
    name: "u8".into(),
    props: StructPropertyMap::new()
  });
  let string_struct = StructPtr::new(Struct {
    name: "String".into(),
    props: StructPropertyMap::new()
  });

  let action_enum = EnumPtr::new(Enum {
    name: "Action".into(),
    variants: VariantMap::from([
      (0, StructPtr::new(Struct {
        name: "Quit".into(),
        props: StructPropertyMap::new()
      })),
      (1, StructPtr::new(Struct {
        name: "Move".into(),
        props: StructPropertyMap::from([
          ("x".into(), StructProperty {
            prototype: StructPtr::clone(&u8_struct),
            default_value: None
          }),
          ("y".into(), StructProperty {
            prototype: StructPtr::clone(&u8_struct),
            default_value: None
          })
        ])
      })),
      (2, StructPtr::new(Struct {
        name: "Write".into(),
        props: StructPropertyMap::from([
          ("0".into(), StructProperty {
            prototype: StructPtr::clone(&string_struct),
            default_value: None
          })
        ])
      })),
      (3, StructPtr::new(Struct {
        name: "ChangeColor".into(),
        props: StructPropertyMap::from([
          ("0".into(), StructProperty {
            prototype: StructPtr::clone(&u8_struct),
            default_value: None
          }),
          ("1".into(), StructProperty {
            prototype: StructPtr::clone(&u8_struct),
            default_value: None
          }),
          ("2".into(), StructProperty {
            prototype: StructPtr::clone(&u8_struct),
            default_value: None
          }),
        ])
      }))
    ])
  });

  let action = Variant {
    prototype: EnumPtr::clone(&action_enum),
    variant_id: 0,
    value: action_enum.variants[&0].new_instance(Map::new())
  };

  dbg!(&action);
}