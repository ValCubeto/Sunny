use super::{Expr, Op, Parser};

parser_method! { fn parse_value() -> Expr }

fn parse_value(parser: &mut Parser) -> Expr {
  let mut val = match parser.current() {
    '-' => Expr::Unary(Op::Neg, parser.parse_value().ptr()),
    '!' => Expr::Unary(Op::Not, parser.parse_value().ptr()),
    '&' => Expr::Unary(Op::Ref, parser.parse_value().ptr()),
    '*' => Expr::Unary(Op::Deref, parser.parse_value().ptr()),
    // '(' => Expr::Single(parser.tuple_or_func()),
    // '\'' => parser.parse_char().ptr(),
    // '"' => parser.parse_string().ptr(),
    // 'f' if parser.peek() == '"' => Expr::Single(parser.parse_fmt_string()),
    // 'r' if parser.peek() == '"' => Expr::Single(parser.parse_raw_string()),
    // 'c' if parser.peek() == '"' => Expr::Single(parser.parse_c_string()),
    // '0'..='9' => Expr::Single(parser.parse_number()),
    // ch if ch.is_alphabetic() => Expr::Single(parser.parse_identifier()),
    ch => syntax_err!("unexpected token {:?}", ch; parser)
  };
  postfix(parser, val)
}

fn postfix(parser: &mut Parser, curr: Expr) -> Expr {
  // parse correctly `x?????`, `x()()()`, `x::y::z`, etc
  match parser.current() {
    '?' => {
      parser.next_token();
      Expr::Unary(Op::Try, postfix(parser, curr).ptr())
    }
    // ':' => {
    //   parser.next_char();
    //   if parser.current() != ':' {
    //     syntax_err!("expected `::`"; parser);
    //   }
    //   let item = parser.parse_ident();
    //   Expr::Arith(Op::GetItem, curr.ptr(), Expr::Single(item).ptr())
    // }
    // '.' if parser.peek() != '.' => {
    //   parser.next_token();
    //   let prop = parser.parse_ident();
    //   Expr::Arith(Op::GetProp, curr.ptr(), Expr::Single(prop).ptr())
    // }
    // '(' => {
    //   parser.next_token();
    //   let args = parser.parse_args();
    //   Expr::Single(Intermediate::Call(curr, args))
    // }
    _ => curr
  }
}

#[derive(Debug)]
pub enum Intermediate {
  String(String),
  Number(String),
  Identifier(String),
  // Call(Expr, Args),
}
