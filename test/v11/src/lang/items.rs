use super::Value;

pub enum Item {
  Constant(Constant),
  // Function(Function),
  // Alias(Alias),
  // Macro(Macro),
  // Trait(Trait),
  // Class(Class),
  // Struct(Struct),
  // Enum(Enum),
  // Bitset(Bitset),
}

pub struct Constant {
  pub ty: Type,
  pub val: Value
}
