use crate::eval::tokenize::tokens::{ Token, Tokens };
use super::{constants::Variable, expressions::Expr, items::{ Entity, Item, Metadata }, types::Type, values::Value};

#[allow(unused)]
#[derive(Debug)]
pub struct Param {
  name: String,
  typing: Type,
  def_value: Option<Expr>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Function {
  name: String,
  params: Vec<Param>,
  generics: Vec<Param>,
  output: Type,
  body: Vec<Expr>,
}

pub fn parse_function(metadata: Metadata, tokens: &mut Tokens) -> Entity {
  if metadata.is_async {
    syntax_err!("async functions not yet implemented");
  }
  if metadata.is_unsafe {
    syntax_err!("unsafe functions not yet implemented");
  }
  let name: String = match tokens.next() {
    Some(Token::Ident(name)) => name.clone(),
    _ => syntax_err!("expected function name")
  };
  let mut params = Vec::new();
  let mut generics = Vec::new();
  let Some(output) = Type::parse(tokens) else {
    syntax_err!("expected function return type");
  };
  let mut body = Vec::new();
  let function = Function {
    name,
    params,
    generics,
    output,
    body
  };
  Entity {
    metadata,
    item: Item::Variable(Variable {
      name,
      typing: Type::from_function(&function),
      value: Expr::Single(Value::Function(function))
    })
  }
}
