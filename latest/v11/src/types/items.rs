use super::Value;

pub enum Item {
  Constant(Constant),
  // Alias(Alias),
  // Macro(Macro),
  // Trait(Trait),
  // Class(Class),
  // Struct(Struct),
  // Enum(Enum),
  // Bitset(Bitset),
}

pub enum BuiltInType {
  Reference(*const str),
  Unknown,

  ClassInstance(*const ()),
  StructInstance(*const ()),
  EnumVariant(*const (), u8),
  BitsetInstance(*const ()),

  Boolean,
  Uint8,
  Uint16,
  UInt32,
  UInt64,
  Usize,
  Int8,
  Int16,
  Int32,
  Int64,
  Isize,
  Float32,
  Float64,
  String,
}

pub struct Constant {
  pub ty: BuiltInType,
  pub val: Value
}
