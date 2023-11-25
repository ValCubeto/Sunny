use crate::{
  types::{ StringPtr, StructPropertyMap, StructPtr },
  values::Value
};

#[derive(Debug)]
pub struct Struct {
  pub name: StringPtr,
  // sorted map, fast search
  pub props: StructPropertyMap
}

impl Struct {
  pub fn has_named_keys(&self) -> bool {
    let first_key = self.props.keys().next().unwrap();
    !first_key.chars().next().unwrap().is_ascii_digit()
  }
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::from("struct ");
    string.push_str(&self.name);
    if !self.props.is_empty() {

      let are_keys_named = self.has_named_keys();
      if are_keys_named {
        string.push_str(" {\n");
      } else {
        string.push('(');
      }
      for (prop_name, prop) in self.props.iter() {
        if are_keys_named {
          string.push_str(&"  ".repeat(depth));
          string.push_str(prop_name);
          string.push_str(": ");
        }
        string.push_str(&prop.prototype.name);
        if prop.default_value.is_some() {
          string.push_str(" = ");
          string.push_str(&prop.default_value.as_ref().unwrap().debug(depth + 1))
        }
        if are_keys_named {
          string.push('\n');
        } else {
          string.push_str(", ");
        }
      }
      if are_keys_named {
        string.push_str(&"  ".repeat(depth - 1));
        string.push('}');
      } else {
        string.push(')');
      }
    }
    string
  }
}

impl PartialEq for Struct {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    // each struct has its own name, even if they have the same name
    StringPtr::ptr_eq(&self.name, &other.name)
  }
}

#[derive(Debug)]
pub struct StructProperty {
  pub prototype: StructPtr,
  pub default_value: Option<Value>
}