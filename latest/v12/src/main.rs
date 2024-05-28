#[macro_use]
mod macros;
#[allow(unused)]
mod parse;

use crate::parse::Parser;

fn main() {
  let mut file = include_str!("../files/test.sny")
    .trim_matches(Parser::is_space)
    .to_owned();
  file.push('\0');
  println!("Data: {:?}", file);
  println!();

  #[allow(clippy::needless_borrow)]
  let mut parser = Parser::new(&file);
  parser.next_char();
  parse_expr(&mut parser);
}

pub fn parse_expr(parser: &mut Parser) {
  parser.next_token();
  println!("[idx={}] Parsing expr starting with: {:?}", parser.idx(), parser.current());
  let val = parse_value(parser);
  println!("{:?}", val);
}

pub fn parse_value(parser: &mut Parser) -> Expr {
  parser.next_token();
  match parser.current() {
    '&' => {
      parser.next_char();
      Expr::Ref(parse_value(parser).ptr())
    },
    '*' => {
      parser.next_char();
      Expr::Deref(parse_value(parser).ptr())
    },
    '0'..='9' => {
      let mut n = String::from(parser.current());
      parser.next_char();
      while parser.current().is_ascii_digit() {
        n.push(parser.current());
        parser.next_char();
      }
      Expr::Token(Value::Number(n))
    },
    _ => panic!("unexpected token {:?}", parser.current())
  }
}

#[derive(Debug)]
pub enum Value {
  Number(String)
}

#[derive(Debug)]
pub enum Expr {
  Token(Value),
  Ref(Box<Expr>),
  Deref(Box<Expr>)
}
impl Expr {
  #[inline(always)]
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}
