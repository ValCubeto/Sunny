use std::fmt;
use super::constants::Variable;

#[allow(unused)]
#[derive(Debug)]
/// Any statement
pub enum Item {
  Variable(Variable),
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
      Item::Variable(variable) => write!(f, "{variable}"),
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
  pub is_unsafe: bool,
  pub is_async: bool,
  pub is_const: bool,
}

#[allow(unused)]
impl Metadata {
  #[inline]
  pub fn default() -> Self {
    Metadata {
      hidden: false,
      mutable: false,
      is_unsafe: false,
      is_async: false,
      is_const: false
    }
  }
}

impl fmt::Display for Metadata {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(if self.hidden { "hidden" } else { "shared" })?;
    f.write_str(if self.mutable { " mutable" } else { " constant" })
  }
}
