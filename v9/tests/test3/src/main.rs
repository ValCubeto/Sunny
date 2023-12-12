use std::{collections::HashMap, rc::Rc};

fn main() {
  let none_class = Class {};
  let string_class = Class {};
  let main_func = Function {
    name: "main",
    generics: HashMap::new(),
    takes_self: false,
    args: Rc::new([]),
    output: Type::Class(&none_class),
    body: FunctionBody::Defined(Rc::new([
      Statement::Define {
        path: Rc::new(["name"]),
        value: Variable {
          mutable: false,
          class: &string_class,
          value: Expression::LiteralString("world")
        }
      },
      Statement::Call {
        path: Rc::new(["println"]),
        args: Rc::new([
          Expression::FormatString(Rc::new([
            FormatStringComponent::Literal("Hello, "),
            FormatStringComponent::Expression(Expression::Identifier(Rc::new(["name"])))
          ]))
        ])
      }
    ]))
  };
  dbg!(&main_func);
  let mut global: HashMap<*const str, Variable> = HashMap::new();
  main_func.run(&mut global, HashMap::new(), Rc::new([]));
}

pub type StringPtr = *const str;
pub type SlicePtr<T> = Rc<[T]>;
pub type ClassPtr = *const Class;

pub struct Class {
  //
}

#[derive(Debug)]
pub enum Type {
  Class(ClassPtr)
}

#[derive(Debug)]
enum Value {
  Function(Function),
  String(String)
}

#[derive(Debug)]
struct Instance {}

#[derive(Debug)]
struct Function {
  name: StringPtr,
  generics: HashMap<StringPtr, Type>,
  takes_self: bool,
  args: SlicePtr<Type>,
  output: Type,
  body: FunctionBody
}

impl Function {
  pub fn run(&self, global: &mut HashMap<StringPtr, Variable>, generics: HashMap<StringPtr, Type>, args: SlicePtr<Instance>) {
    dbg!(&generics, &args);
    match &self.body {
      FunctionBody::Defined(body) => {
        for statement in body.iter() {
          match statement {
            Statement::Define { path, value } => {
              global.insert(path[0], value.clone());
            },
            Statement::Call { path, args } => {}
          }
        }
      },
      _ => unimplemented!()
    }
  }
}

#[derive(Debug)]
enum FunctionBody {
  // Builtin(fn(Args) -> Instance),
  Defined(SlicePtr<Statement>)
}

#[derive(Debug)]
enum Statement {
  Define {
    path: SlicePtr<StringPtr>,
    value: Variable
  },
  Call {
    path: SlicePtr<StringPtr>,
    args: SlicePtr<Expression>
  }
}

#[derive(Debug, Clone)]
struct Variable {
  mutable: bool,
  class: ClassPtr,
  value: Expression
}

#[derive(Debug, Clone)]
enum Expression {
  Identifier(SlicePtr<StringPtr>),
  LiteralString(StringPtr),
  FormatString(SlicePtr<FormatStringComponent>)
}

#[derive(Debug)]
enum FormatStringComponent {
  Literal(StringPtr),
  Expression(Expression)
}
  