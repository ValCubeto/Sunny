use std::rc::Rc;
use crate::{
  types::{ SlicePtr, StringPtr, Map, EnumPtr, ClassPtr },
  values::Value, instances::Instance
};

pub type Arguments = SlicePtr<Type>;
pub type BuiltinFunctionPtr = Rc<dyn Fn(Arguments) -> Value>;

pub struct Function {
  pub gets_self: bool,
  pub name: StringPtr,
  pub generics: Generics,
  pub params: SlicePtr<Type>, // (T, A, B, C)
  pub output: Type,
  pub body: FunctionBody,
}

impl std::fmt::Debug for Function {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let generics = self.generics.keys().cloned().collect::<Vec<_>>().join(", ");
    let mut params = String::new();
    let mut params_iter = self.params.iter();
    let p = params_iter.next().unwrap();
    params.push_str(&format!("{p:?}"));
    for p in params_iter {
      params.push_str(&format!(", {p:?}"));
    }
    write!(f, "fun {}{}({}) -> {:?}", self.name, generics, params, self.output)
  }
}

pub type FunctionPtr = Rc<Function>;

pub enum FunctionBody {
  Builtin(BuiltinFunctionPtr),
  Declared(SlicePtr<Statement>)
}

pub type Block = SlicePtr<Statement>;

pub enum Statement {
  // If(Expr, Block, Block)
}

pub enum Type {
  Variant(EnumPtr),
  Instance(ClassPtr),
  Implements(SlicePtr<TraitPtr>)
}

impl std::fmt::Debug for Type {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Variant(variant) => write!(f, "{}", variant.name),
      Self::Instance(instance) => write!(f, "{}", instance.name),
      Self::Implements(traits) => {
        let mut traits_iter = traits.iter();
        let mut traits = String::new();
        let t = traits_iter.next().unwrap();
        traits.push_str(&format!("{t:?}"));
        for t in traits_iter {
          traits.push_str(&format!(" + {t:?}"));
        }
        write!(f, "impl {traits:?}")
      }
    }
  }
}

// names and default values
pub type Generics = Map<Option<Type>>;

pub type TraitPtr = Rc<Trait>;

/// example: trait Number: Sum<Self> + ... { const MIN: Self, const MAX: Self }

pub struct Trait {
  pub name: StringPtr,
  pub generics: Generics,
  // TODO: this only applies to Self. change it to a HashMap
  pub requeriments: SlicePtr<TraitPtr>,
  // TODO: change this to Map<HashMap<Constant>>
  pub constants: Map<Instance>, // require const & fun
  pub methods: Map<Function>
}

impl std::fmt::Debug for Trait {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut generics = String::new();
    let mut generics_iter = self.generics.values();
    let g = generics_iter.next().unwrap();
    generics.push_str(&format!("{g:?}"));
    for g in generics_iter {
      generics.push_str(&format!(", {g:?}"));
    }
    write!(f, "{}<{}>", self.name, generics)
  }
}

#[derive(Debug)]
pub struct Constant {
  name: StringPtr,
  type_of: Type,
  value: Value
}