use std::fmt;
use super::constants::Variable;

#[allow(unused)]
#[derive(Debug)]
/// Any statement
pub enum Item {
  Const(Variable),
  // Struct,
  // Enum,
  // BitSet,
  // Idea,
  // TypeDef,
  // Import,
  // Macro,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Entity {
  pub metadata: Metadata,
  pub item: Item,
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    #[allow(unreachable_patterns, reason="Not all variants are implemented yet")]
    match self {
      Item::Const(variable) => write!(f, "{}", variable),
      // Item::Struct => write!(f, "struct"),
      // Item::Enum => write!(f, "enum"),
      // Item::BitSet => write!(f, "bitset"),
      // Item::Idea => write!(f, "idea"),
      // Item::TypeDef => write!(f, "typedef"),
      // Item::Import => write!(f, "import"),
      // Item::Macro => write!(f, "macro"),
      _ => unimplemented!()
    }
  }
}

impl fmt::Display for Entity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} ", if self.metadata.public() { "pub" } else { "priv" })?;
    write!(f, "{} ", if self.metadata.mutable() { "mut" } else { "const" })?;
    write!(f, "{}", self.item)
  }
}

type M = u8;

// This is a bit set btw
pub struct Metadata(M);
#[allow(unused)]
impl Metadata {
  pub const TRUE:  M = 1;
  pub const FALSE: M = 0;
  // The shiftings
  pub const IS_PUBLIC : M = 0;
  pub const IS_MUTABLE: M = 1;

  #[inline]
  pub fn new() -> Self {
    Metadata(0)
  }
  pub fn public(&self) -> bool {
    self.0 & (1 << Self::IS_PUBLIC) != 0
  }
  pub fn set_public(&mut self, is_public: bool) {
    // 0000_1010 ^ 0000_0001 = 0000_1011
    // 0000_1010 ^ 0000_0000 = 0000_1010
    self.0 ^= (is_public as M) << Self::IS_PUBLIC;
  }
  pub fn mutable(&self) -> bool {
    self.0 & (1 << Self::IS_MUTABLE) != 0
  }
  pub fn set_mutable(&mut self, is_mutable: bool) {
    self.0 ^= (is_mutable as M) << Self::IS_MUTABLE;
  }
}

impl fmt::Debug for Metadata {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Metadata")
      .field("public", &self.public())
      .field("mutable", &self.mutable())
      .finish()
  }
}
