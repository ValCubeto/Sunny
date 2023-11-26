use std::rc::Rc;

use crate::{types::{SlicePtr, StringPtr, Map, EnumPtr, ClassPtr}, values::Value};

pub type Arguments = SlicePtr<Type>;
pub type BuiltinFunctionPtr = Rc<dyn Fn(Arguments) -> Value>;

pub struct Function {
  pub gets_self: bool,
  pub name: StringPtr,
  pub generics: Map<Type>,
  pub params: SlicePtr<Type>,
  pub output: Type,
  pub body: FunctionBody,
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

#[derive(Debug)]
pub enum Type {
  Variant(EnumPtr),
  Instance(ClassPtr),
  Implements(SlicePtr<TraitPtr>)
}

pub type TraitPtr = Rc<Trait>;

#[derive(Debug)]
pub struct Trait {
  name: StringPtr,
  requeriments: SlicePtr<TraitPtr>,
  values: Map<Constant> // require const & fun
}

#[derive(Debug)]
pub struct Constant {
  name: StringPtr,
  type_of: Type,
  value: Value
}