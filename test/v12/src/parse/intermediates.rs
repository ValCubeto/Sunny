use super::{Expr, Op, Parser};

parser_method! { fn parse_value() -> Expr }

fn parse_value(parser: &mut Parser) -> Expr {
  parser.next_token();
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
  parser.next_token();
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

// pub fn tuple_or_func(parser: &mut Parser) -> Expr {
//   parser.next_token();
//   let mut args = HashMap::new();
//   while parser.current() != ')' {
//     if parser.is_ident() {
//       let name = parser.parse_word();
//       parser.next_token();
//       parser.expect(':');
//       parser.next_token();
//       let typing = parser.parse_type();
//       parser.next_token();
//       if parser.current() == '=' {
//         parser.next_token();
//         let value = parser.parse_value(Context::Constant);
//         args.insert(name, Arg { typing, value: Some(value) });
//       } else {
//         args.insert(name, Arg { typing, value: None });
//       }
//       continue;
//     }
//     match parser.current() {
//       '{' => {
//         parser.next_token();
//         let mut fields = HashMap::new();
//         while parser.current() != '}' {
//           parser.next_token();
//           let path = parser.expect_props_path();
//           if parser.current() == ':' {
//             parser.next_token();
//             let typing = parser.parse_type();
//             fields.insert(path, typing);
//           }
//         }
//       },
//       '(' => {},
//       '[' => {},
//       _ => parser.unexpected(),
//     }
//   }
// }

#[derive(Debug, Clone)]
pub enum Intermediate {
  String(String),
  Number(String),
  Identifier(String),
  // Call(Expr, Args),
}
