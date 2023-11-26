use std::rc::Rc;

use crate::{types::{SlicePtr, StringPtr, Map, EnumPtr, ClassPtr}, values::Value};

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
    let params = self.params.join(", ");
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
        write!(f, "{}", traits.iter().map(|t| {
          format!("{}{}", t.name, t.generics.iter())
        }).collect::<Vec<_>>().join(" + "))
      }
    }
  }
}

// names and default values
pub type Generics = Map<Option<Type>>;

pub type TraitPtr = Rc<Trait>;

/// example: trait Number: Sum<Self> + ... { const MIN: Self, const MAX: Self }
#[derive(Debug)]
pub struct Trait {
  name: StringPtr,
  generics: Generics,
  requeriments: SlicePtr<TraitPtr>,
  values: Map<Constant> // require const & fun
}

#[derive(Debug)]
pub struct Constant {
  name: StringPtr,
  type_of: Type,
  value: Value
}