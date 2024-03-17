
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
  #[allow(unused)]
  pub fn to_string(&self, depth: usize) -> String {
    use Value as V;
    match self {
      V::String(s) => format!("{s:?}"),
      V::Id(s) => format!("{s:?}"),
      V::Array(_) | V::Vec(_) => {
        let mut vec = match self {
          V::Array(a) => a.to_vec(),
          V::Vec(v) => v.clone(),
          _ => unreachable!()
        };
        let mut string = String::with_capacity(2);
        string.push('[');
        if !vec.is_empty() {
          let last = vec.remove(vec.len() - 1);
          const IDENT: &str = "    ";
          for value in vec {
            let line = format!("{}{},\n", IDENT.repeat(depth), value.to_string(depth + 1));
            string.push_str(line.as_str());
          }
          let line = format!("{}{}\n", IDENT.repeat(depth), last.to_string(depth + 1));
          string.push_str(line.as_str());
        }
        string.push(']');
        string
      },
      _ => todo!()
    }
  }
}