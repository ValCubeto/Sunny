use crate::{
  types::EnumPtr,
  instances::Instance
};


#[derive(Debug)]
pub struct Variant {
  pub prototype: EnumPtr,
  pub variant_id: usize,
  pub value: Instance
}

impl Variant {
  #[allow(clippy::needless_return)]
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::new();
    string.push_str(&self.prototype.name);
    string.push_str("::");
    string.push_str(&self.value.prototype.name);
    if self.value.props.is_empty() {
      return string;
    }
    if self.value.prototype.has_numeric_keys() {
      string.push('(');
      todo!();
    }
    string.push_str(" {");
    for (prop_name, value) in self.value.prototype.props.keys().zip(self.value.props.iter()) {
      string.push('\n');
      string.push_str(&"  ".repeat(depth));
      string.push_str(prop_name);
      string.push_str(" = ");
      string.push_str(&value.debug(depth + 1));
    }
    string.push('\n');
    string.push_str(&"  ".repeat(depth - 1));
    string.push('}');
    return string;
  }
}
