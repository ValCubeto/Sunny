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
    #[allow(unreachable_patterns)]
    match self {
      Item::Const(variable) => write!(f, "{variable}"),
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
    write!(f, "{} {}", self.metadata, self.item)
  }
}

#[derive(Debug, Clone, Copy)]
/// TODO: turn this into a bit set
pub struct Metadata {
  pub hidden: bool,
  pub mutable: bool,
}

#[allow(unused)]
impl Metadata {
  #[inline]
  pub fn new() -> Self {
    Metadata {
      hidden: false,
      mutable: false,
    }
  }
}

impl fmt::Display for Metadata {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(if self.hidden { "hidden" } else { "shared" })?;
    f.write_str(if self.mutable { " mutable" } else { " constant" })
  }
}
