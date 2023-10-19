extern crate rand;
extern crate num_bigint;
extern crate bigdecimal;

mod colors;
mod about;
mod macros;
mod aliases;
mod values;
mod context;
mod args;
mod commands;
mod table;

use args::parse_args;
use context::tokens::Node;

use crate::{context::tokens::{build_ast, Token, Operator}, values::Value};

pub fn main() {
  let tokens = vec![
    Token::Value(Value::u8(2)),
    Token::Op(Operator::Mul),
    Token::Value(Value::u8(3)),
    Token::Op(Operator::Add),
    Token::Value(Value::u8(1)),
  ];
  let ast = build_ast(tokens);
  debug!("ast = {ast:?}");
  let res = eval_ast(ast);
  debug!("res = {res:?}");
  todo!();
  parse_args();
}

fn eval_ast(ast: Node) -> Value {
  match ast {
    Node::Value(value) => value,
    Node::Op(op, a, b) => {
      match op {
        Operator::Add => {
          let left = eval_ast(*a);
          let right = eval_ast(*b);
          match left {
            Value::u8(a) => {
              match right {
                Value::u8(b) => Value::u8(a + b),
                _ => todo!()
              }
            },
            _ => todo!()
          }
        }
        Operator::Mul => {
          let left = eval_ast(*a);
          let right = eval_ast(*b);
          match left {
            Value::u8(a) => {
              match right {
                Value::u8(b) => Value::u8(a * b),
                _ => todo!()
              }
            },
            _ => todo!()
          }
        },
        _ => todo!()
      }
    }
  }
}