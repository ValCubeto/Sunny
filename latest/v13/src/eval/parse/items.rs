use std::fmt::Display;
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
  A
}

#[allow(unused)]
#[derive(Debug)]
pub struct Entity {
  pub metadata: Metadata,
  pub item: Item,
}

impl Display for Item {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Item::Const(variable) => write!(f, "const {}", variable),
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

impl Display for Entity {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.metadata.public() {
      write!(f, "pub ")?;
    }
    if self.metadata.mutable() {
      write!(f, "mut ")?;
    }
    write!(f, "{}", self.item)
  }
}

// This is a bit set btw
type M = u8;
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
    Metadata(0).set_public(Self::TRUE)
  }
  pub fn public(&self) -> bool {
    self.0 & (1 << Self::IS_PUBLIC) != 0
  }
  pub fn set_public(mut self, is_public: M) -> Self {
    // 0000_1010 ^ 0000_0001 = 0000_1011
    // 0000_1010 ^ 0000_0000 = 0000_1010
    self.0 ^= is_public << Self::IS_PUBLIC;
    self
  }
  pub fn mutable(&self) -> bool {
    self.0 & (1 << Self::IS_MUTABLE) != 0
  }
  pub fn set_mutable(mut self, is_mutable: M) -> Self {
    self.0 ^= is_mutable << Self::IS_MUTABLE;
    self
  }
}
impl std::fmt::Debug for Metadata {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Metadata")
      .field("public", &self.public())
      .field("mutable", &self.mutable())
      .finish()
  }
}
