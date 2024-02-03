use crate::{
  types::{ ClassPtr, SlicePtr, StringPtr, Map },
  values::Value
};

#[derive(Clone, Debug)]
pub struct Instance {
  pub prototype: ClassPtr,
  pub props: SlicePtr<Value>
}

impl Instance {
  #[allow(clippy::needless_return)]
  pub fn debug(&self, depth: usize) -> String {
    let mut string = String::new();
    string.push_str(&self.prototype.name);
    if self.prototype.props.is_empty() {
      return string;
    }
    if self.prototype.has_numeric_keys() {
      string.push('(');
      // string.push_str("(\n");
      string.push(')');
      todo!();
      return string;
    }
    string.push_str(" {\n");
    for (key, value) in self.prototype.props.keys().zip(self.props.iter()) {
      string.push_str(&"  ".repeat(depth));
      string.push_str(key);
      string.push_str(" = ");
      string.push_str(&value.debug(depth + 1));
    }
    string.push_str(&"  ".repeat(depth - 1));
    string.push('}');
    return string;
  }

  // TODO: i should cache
  pub fn get_property(&mut self, key: StringPtr) -> &mut Value {
    let search = self.prototype.props.iter().position(|(curr_key, _)| *curr_key == key);
    let index = match search {
      Some(index) => index,
      None => panic!("'{}' has no property '{}'", self.prototype.name, key),
    };
    &mut self.props[index]
  }
  pub fn set_property(&mut self, key: StringPtr,new_value: Value) {
    *self.get_property(key) = new_value;
  }
  // pub fn try_get_property(&self, key: StringPtr) -> Option<Value> {
  //   let search = self.prototype.props.iter().position(|(curr_key, _)| *curr_key == key);
  //   let index = match search {
  //     Some(index) => index,
  //     None => return None
  //   };
  //   Some(self.props[index].clone())
  // }
}

pub trait CreateInstance {
  fn new_instance(&self, props: Map<Instance>) -> Instance;
}
impl CreateInstance for ClassPtr {
  fn new_instance(&self, mut candidates: Map<Instance>) -> Instance {
    let mut props = Vec::with_capacity(self.props.len());
    for (key, prop) in self.props.iter() {
      // this does not deallocate
      match candidates.remove(key) {
        Some(instance) => {
          if instance.prototype != prop.prototype {
            panic!(
              "Mismatched types. '{}.{}' has type '{}', found '{}'.",
              self.name, key, prop.prototype.name, instance.prototype.name
            );
          };
          props.push(Value::Instance(instance));
        },
        None => {
          match &(prop.default_value) {
            Some(default_value) => {
              props.push(default_value.clone());
            }
            None => panic!("Missing property '{}' to create a '{}'", key, self.name)
          }
        }
      }
    }
    if !candidates.is_empty() {
      if candidates.len() == 1 {
        let key = candidates.keys().next().unwrap();
        panic!("'{}' has no property '{}'", self.name, key);
      }
      let keys = candidates.keys()
        // .take(3)
        .cloned()
        .collect::<Vec<_>>()
        .join("', '");
      panic!("'{}' has no properties '{}'", self.name, keys);
    }
    Instance {
      prototype: ClassPtr::clone(self),
      props: props.into_boxed_slice()
    }
  }
}