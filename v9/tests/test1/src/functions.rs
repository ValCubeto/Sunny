pub enum FunctionPtr {
  Builtin(BuiltinFunctionPtr),
  Declared(DeclaredFunctionPtr)
}

pub type Arguments = SlicePtr<Type>;
pub type BuiltinFunctionPtr = Rc<dyn Fn(Arguments) -> Value>;
pub type DeclaredFunctionPtr = Rc<Function>;

pub struct Function {
  pub name: StringPtr,
  pub generics: Map<Type>,
  pub params: SlicePtr<Type>,
  pub output: Type,
  pub body: SlicePtr<Statement>,
}
pub enum Type {
  Variant(EnumPtr),
  Instance(ClassPtr),
  Implements(Trait)
}
