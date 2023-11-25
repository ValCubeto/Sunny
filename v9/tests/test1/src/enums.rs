use crate::{
  types::{ Pointer, VariantMap, StringPtr },
  instances::Instance
};

#[derive(Debug)]
pub struct Enum {
  pub name: StringPtr,
  pub variants: VariantMap
}

impl Enum {
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::from("enum ");
    string.push_str(&self.name);
    if !self.variants.is_empty() {
      string.push_str(" {\n");
      for (discriminator, prototype) in self.variants.iter() {
        string.push_str(&"  ".repeat(depth));
        string.push_str(&discriminator.to_string());
        string.push_str(": ");
        string.push_str(&prototype.debug(depth + 1));
        string.push('\n');
      }
      string.push_str(&"  ".repeat(depth - 1));
      string.push('}');
    }
    string
  }
}
