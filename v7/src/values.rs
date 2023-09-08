
use {
  std::rc::Rc,
  crate::{
    aliases::Dict,
    aliases::Id,
    functions::Function,
    namespaces::Namespace,
    structs::Struct,
    instances::Instance
  }
};

#[allow(unused)]
#[derive(Clone, PartialEq, Eq)]
pub enum Value {
  Null,
  String(String),
  Id(Id),
  Vec(Vec<Value>),
  Array(Rc<[Value]>),
  Dict(Rc<Dict>),
  Function(Rc<Function>),
  Namespace(Rc<Namespace>),
  Struct(Rc<Struct>),
  Instance(Rc<Instance>),
}

impl Value {
  pub fn to_string(&self, depth: usize) -> String {
    use Value as V;
    match self {
      V::String(s) => format!("{s:?}"),
      V::Id(s) => format!("{s:?}"),
      V::Array(a) => {
        let string = String::with_capacity(2);
        string.push('[');
        if !a.is_empty() {
          let vec = a.to_vec();
          let last = vec.remove(vec.len() - 1);
          for v in vec {
            string.push_str((v.to_string(depth + 1) + ",\n").as_str())
          }
        }
        string.push(']');
        string
      }
    }
  }
}