use crate::{
  types::{ StringPtr, ClassPropertyMap, ClassPtr, SlicePtr },
  values::Value, functions::Constant
};

#[derive(Debug)]
pub struct Class {
  pub name: StringPtr,
  pub props: ClassPropertyMap,
  pub values: SlicePtr<Constant>
}

impl Class {
  #[inline]
  pub fn has_named_keys(&self) -> bool {
    self.props
      .keys().next().unwrap()
      .chars().next().unwrap()
      .is_ascii_digit()
  }
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::from("Class ");
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

impl PartialEq for Class {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    // each Class has its own name, even if they have the same name
    StringPtr::ptr_eq(&self.name, &other.name)
  }
}

#[derive(Debug)]
pub struct ClassProperty {
  pub prototype: ClassPtr,
  pub default_value: Option<Value>
}