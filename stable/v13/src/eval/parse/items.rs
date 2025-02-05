use std::fmt;
use super::constants::Variable;

#[derive(Debug)]
/// Any statement
pub enum Item {
  Variable(Variable),
  // Import,
  // Struct,
  // Enum,
  // BitSet,
  // Interface,
  // TypeDef,
  // Macro,
}

#[derive(Debug)]
pub struct Entity {
  pub metadata: Metadata,
  pub item: Item,
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Item::Variable(variable) => write!(f, "{variable}"),
      // Item::Struct => write!(f, "struct"),
      // Item::Enum => write!(f, "enum"),
      // Item::BitSet => write!(f, "bitset"),
      // Item::TypeDef => write!(f, "typedef"),
      // Item::Import => write!(f, "import"),
      // Item::Macro => write!(f, "macro"),
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
  pub is_static: bool,
}

impl Metadata {
  #[inline]
  pub fn default() -> Self {
    Metadata {
      hidden: false,
      mutable: false,
      is_unsafe: false,
      is_async: false,
      is_static: false,
    }
  }
}

impl fmt::Display for Metadata {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut meta = Vec::new();
    if self.is_unsafe {
      meta.push("unsafe");
    }
    if self.is_static {
      meta.push("static");
    }
    if self.is_async {
      meta.push("async");
    }
    meta.push(if self.hidden { "hidden" } else { "shared" });
    meta.push(if self.mutable { "mutable" } else { "constant" });
    write!(f, "{}", meta.join(" "))
  }
}
