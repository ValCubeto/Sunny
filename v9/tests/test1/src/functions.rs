pub type Arguments = SlicePtr<Type>;
pub type BuiltinFunctionPtr = Rc<dyn Fn(Arguments) -> Value>;

pub struct Function {
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

pub enum Type {
  Variant(EnumPtr),
  Instance(ClassPtr),
  Implements(SlicePtr<TraitPtr>)
}

pub type TraitPtr = Rc<Trait>;

pub struct Trait {
  name: StringPtr,
  requeriments: SlicePtr<TraitPtr>,
  values: Map<Constant> // require const & fun
}

pub struct Constant {
  name: StringPtr,
  type_of: Type,
}