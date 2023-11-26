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
  #[allow(clippy::expect_fun_call)]
  pub fn has_numeric_keys(&self) -> bool {
    self.props
      .keys().next()
      .expect(&format!("Internal error: se intentó acceder a las \
                        propiedades de la clase {:?}, pero esta \
                        no tenía ninguna.", self.name))
      .chars().next()
      .expect(&format!("Internal error: The class {:?} has a \
                        property with an empty name.", self.name))
      .is_ascii_digit()
  }
  #[allow(clippy::needless_return)]
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::from("struct ");
    string.push_str(&self.name);
    if self.props.is_empty() {
      return string;
    }
    if self.has_numeric_keys() {
      string.push('(');
      let mut props = self.props.values();
      let prop = props.next().unwrap();
      string.push_str(&prop.prototype.name);
      for prop in props {
        string.push_str(", ");
        string.push_str(&prop.prototype.name);
      }
      string.push(')');
      return string;
    }
    string.push_str(" {\n");
    let mut props = self.props.iter();
    let (prop_name, prop) = props.next().unwrap();
    string.push_str(&"  ".repeat(depth));
    string.push_str(prop_name);
    string.push_str(": ");
    string.push_str(&prop.prototype.name);
    if prop.default_value.is_some() {
      string.push_str(" = ");
      string.push_str(&prop.default_value.as_ref().unwrap().debug(depth + 1))
    }
    for (prop_name, prop) in self.props.iter().skip(1) {
      string.push_str(",\n");
      string.push_str(&"  ".repeat(depth));
      string.push_str(prop_name);
      string.push_str(": ");
      string.push_str(&prop.prototype.name);
      if prop.default_value.is_some() {
        string.push_str(" = ");
        string.push_str(&prop.default_value.as_ref().unwrap().debug(depth + 1))
      }
    }
    string.push('\n');
    string.push_str(&"  ".repeat(depth - 1));
    string.push('}');
    return string;
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