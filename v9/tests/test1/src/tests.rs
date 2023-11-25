use crate::{
  types::*,
  values::*,
  classes::*,
  instances::*,
  enums::*,
  variants::*,
};

pub fn test_classes() {
  println!("[test::Classs]");
  let u8_class = ClassPtr::new(Class {
    name: "u8".into(),
    props: ClassPropertyMap::new()
  });

  // Class Point { x: u8, y: u8 }
  let point_class = ClassPtr::new(Class {
    name: "Point".into(),
    props: ClassPropertyMap::from([
      ("x".into(), ClassProperty {
        prototype: ClassPtr::clone(&u8_class),
        default_value: None
      }),
      ("y".into(), ClassProperty {
        prototype: ClassPtr::clone(&u8_class),
        default_value: None
      }),
    ])
  });

  // let point = Point { y: 5, x: 10 } (order does not matter)
  let point_instance = point_class.new_instance(Map::from([
    ("y".into(), Instance {
      prototype: ClassPtr::clone(&u8_class),
      props: Box::new([ Value::Uint8(5) ])
    }),
    ("x".into(), Instance {
      prototype: ClassPtr::clone(&u8_class),
      props: Box::new([ Value::Uint8(10) ])
    }),
  ]));

  let mut stack = Map::from([
    ("Point".into(), Value::Class(ClassPtr::clone(&point_class))),
    ("point".into(), Value::Instance(point_instance))
  ]);

  dbg!(point_class == u8_class);
  match stack.get_mut("point").unwrap() {
    Value::Instance(point) => {
      dbg!(&point.prototype.name);
      println!("{}", point.get_property("x".into()).debug(1));
      // point.x = 20
      point.set_property("x".into(), Value::Instance(Instance {
        prototype: ClassPtr::clone(&u8_class),
        props: Box::new([ Value::Uint8(20) ])
      }));
      println!("{}", point.get_property("x".into()).debug(1));
    }
    _ => unreachable!()
  };

  println!();
}





pub fn test_enums() {
  println!("[test::enums]");
  let u8_class = ClassPtr::new(Class {
    name: "u8".into(),
    props: ClassPropertyMap::new()
  });
  let string_class = ClassPtr::new(Class {
    name: "String".into(),
    props: ClassPropertyMap::new()
  });

  let action_enum = EnumPtr::new(Enum {
    name: "Action".into(),
    variants: VariantMap::from([
      (0, ClassPtr::new(Class {
        name: "Quit".into(),
        props: ClassPropertyMap::new()
      })),
      (1, ClassPtr::new(Class {
        name: "Move".into(),
        props: ClassPropertyMap::from([
          ("x".into(), ClassProperty {
            prototype: ClassPtr::clone(&u8_class),
            default_value: None
          }),
          ("y".into(), ClassProperty {
            prototype: ClassPtr::clone(&u8_class),
            default_value: None
          })
        ])
      })),
      (2, ClassPtr::new(Class {
        name: "Write".into(),
        props: ClassPropertyMap::from([
          ("0".into(), ClassProperty {
            prototype: ClassPtr::clone(&string_class),
            default_value: None
          })
        ])
      })),
      (3, ClassPtr::new(Class {
        name: "ChangeColor".into(),
        props: ClassPropertyMap::from([
          ("0".into(), ClassProperty {
            prototype: ClassPtr::clone(&u8_class),
            default_value: None
          }),
          ("1".into(), ClassProperty {
            prototype: ClassPtr::clone(&u8_class),
            default_value: None
          }),
          ("2".into(), ClassProperty {
            prototype: ClassPtr::clone(&u8_class),
            default_value: None
          }),
        ])
      }))
    ])
  });

  let action = Variant {
    prototype: EnumPtr::clone(&action_enum),
    variant_id: 1,
    value: action_enum.variants[&1].new_instance(Map::from([
      ("x".into(), Instance {
        prototype: ClassPtr::clone(&u8_class),
        props: vec![ Value::Uint8(50) ].into_boxed_slice()
      }),
      ("y".into(), Instance {
        prototype: ClassPtr::clone(&u8_class),
        props: vec![ Value::Uint8(10) ].into_boxed_slice()
      }),
    ]))
  };

  println!("{}", action_enum.debug(1));
  println!("{}", action.debug(1));

  println!();
}

