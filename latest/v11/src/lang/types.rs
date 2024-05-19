pub enum BuiltInType {
  Inferred,
  Reference(*const str),

  ClassInstance(*const ()),
  StructInstance(*const ()),
  EnumVariant(*const ()),
  BitsetInstance(*const ()),

  Pointer,

  Bool,

  Int8,
  Int16,
  Int32,
  Int64,
  Int128,
  ISize,

  UInt8,
  UInt16,
  UInt32,
  UInt64,
  UInt128,
  USize,

  Float32,
  Float64,
  Float128,

  Char,

  List,
  String,
}

pub enum Type {
  BuiltInType(BuiltInType),
  // Function(Function),
  // Alias(Alias),
  // Macro(Macro),
  // Trait(Trait),
  // Class(Class),
  // Struct(Struct),
  // Enum(Enum),
  // Bitset(Bitset),
}
