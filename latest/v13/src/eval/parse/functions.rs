use crate::eval::tokenize::keywords::Keyword;
use crate::eval::tokenize::tokens::{ Operator, Token, Tokens };
use crate::eval::parse::{
  constants::Variable,
  expressions::Expr,
  items::{ Entity, Item, Metadata },
  types::Typing,
  values::Value
};

#[allow(unused)]
#[derive(Debug)]
pub struct Param {
  pub name: String,
  pub typing: Typing,
  // Default value
  pub default_val: Option<Expr>,
}

#[allow(unused)]
#[derive(Debug)]
pub struct Function {
  pub name: String,
  pub params: Vec<Param>,
  pub generics: Vec<Param>,
  pub output: Typing,
  pub body: Vec<Expr>,
}

// Some functions have no name
pub fn parse_function(metadata: Metadata, tokens: &mut Tokens, name: String) -> Entity {
  if metadata.is_async {
    syntax_err!("async functions not yet implemented");
  }
  if metadata.is_unsafe {
    syntax_err!("unsafe functions not yet implemented");
  }
  if metadata.is_const {
    syntax_err!("const functions not yet implemented");
  }
  // let mut self_type
  let mut generics = Vec::new();
  if matches!(tokens.peek(), Token::Op(Operator::LeftAngle)) {
    tokens.next();
    syntax_err!("function generics not yet implemented");
  }
  let mut params = Vec::new();
  match tokens.next() {
    Token::LeftParen => {
      #[allow(clippy::never_loop)]
      loop {
        match tokens.peek() {
          Token::RightParen => break,
          Token::Ident(_name) => {
            syntax_err!("function parameters not yet implemented");
          }
          Token::LeftBrace => {
            syntax_err!("parameter destructuring not yet implemented");
          }
          Token::LeftParen => {
            syntax_err!("parameter destructuring not yet implemented");
          }
          Token::LeftBracket => {
            syntax_err!("parameter destructuring not yet implemented");
          }
          _ => syntax_err!("expected parameter list")
        }
      }
    }
    _ => syntax_err!("expected parameters")
  }
  let output = match tokens.peek() {
    Token::Arrow => {
      tokens.next();
      Typing::parse(tokens)
    }
    _ => Typing::Undefined
  };
  if !matches!(tokens.next(), Token::RightParen) {
    syntax_err!("unclosed parenthesis");
  }
  let mut body = Vec::new();
  if matches!(tokens.next(), Token::LeftBrace) {
    parse_stmt(tokens, &mut body);
  }
  let function = Function {
    name: name.clone(),
    params,
    generics,
    output,
    body
  };
  Entity {
    metadata,
    item: Item::Variable(Variable {
      name,
      typing: Typing::from_function(&function),
      value: Expr::Single(Value::Function(function))
    })
  }
}

fn parse_stmt(tokens: &mut Tokens, body: &mut Vec<Expr>) {
  while let Some(token) = tokens.try_next() {
    match token {
      Token::NewLine | Token::Semicolon => continue,
      Token::RightParen => break,
      Token::EoF => syntax_err!("unexpected end of file"),
      Token::Keyword(Keyword::Let) => {
        syntax_err!("let statements not yet implemented");
        // body.push(Expr::parse(tokens));
      }
      Token::Keyword(Keyword::Var) => {
        syntax_err!("var statements not yet implemented");
      }
      Token::Keyword(Keyword::If) => {
        syntax_err!("if statements not yet implemented");
      }
      Token::Keyword(Keyword::Loop) => {
        syntax_err!("loops not yet implemented");
      }
      Token::Keyword(Keyword::While) => {
        syntax_err!("loops not yet implemented");
      }
      Token::Keyword(Keyword::For) => {
        syntax_err!("for loops not yet implemented");
      }
      Token::Keyword(Keyword::Match) => {
        syntax_err!("match statements not yet implemented");
      }
      Token::Keyword(Keyword::Defer) => {
        syntax_err!("defer blocks not yet implemented");
      }
      Token::Keyword(Keyword::Return) => {
        syntax_err!("returns not yet implemented");
      }
      _ => syntax_err!("unexpected {token}")
    }
  }
}
