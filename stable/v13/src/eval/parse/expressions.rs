use std::fmt;
use crate::eval::{
  tokenize::tokens::{ Token as Tk, Tokens },
  parse::{ types::Type, values::Value }
};

type E = Box<Expr>;

#[allow(unused)]
#[derive(Debug)]
pub enum Expr {
  None,
  Single(Value),
  Type(Type),

  /// `!a`
  Not(E),
  /// `-a`
  Neg(E),
  /// `+a`
  Pos(E),
  /// `*a`
  Deref(E),
  /// `&a`
  Ref(E),
  /// `a?`
  Try(E),

  /// `a + b`
  Add(E, E),
  /// `a - b`
  Sub(E, E),
  /// `a * b`
  Mul(E, E),
  /// `a / b`
  Div(E, E),
  /// `a % b`
  Mod(E, E),
  /// `a && b`
  LogicalAnd(E, E),
  /// `a || b`
  LogicalOr(E, E),
  /// `a & b`
  And(E, E),
  /// `a | b`
  Or(E, E),
  /// `a ^ b`
  Xor(E, E),
  /// `a == b`
  Equal(E, E),
  /// `a != b`
  NotEqual(E, E),
  /// `a < b`
  Less(E, E),
  /// `a > b`
  Greater(E, E),
  /// `a <= b`
  LessEqual(E, E),
  /// `a >= b`
  GreaterEqual(E, E),
  /// `a << b`
  LeftShift(E, E),
  /// `a >> b`
  RightShift(E, E),
  /// `a <> b`
  Cmp(E, E),

  /// `a .. b`
  ExclusiveRange(E, E),
  /// `a ... b`
  InclusiveRange(E, E),

  /// `a :: b`
  GetItem(E, E),
  /// `a . b`
  GetField(E, E)
}

impl Expr {
  /// None if there are no tokens
  pub fn parse(tokens: &mut Tokens) -> Expr {
    parse_expr_bp(tokens, 0)
  }

  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}

impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::None => Ok(()),
      Self::Single(value) => write!(f, "{value}"),
      Self::Type(ty) => write!(f, "{ty}"),
      Self::Not(expr) => write!(f, "(!{expr})"),
      Self::Neg(expr) => write!(f, "(-{expr})"),
      Self::Pos(expr) => write!(f, "(+{expr})"),
      Self::Deref(expr) => write!(f, "(*{expr})"),
      Self::Ref(expr) => write!(f, "(&{expr})"),
      Self::Try(expr) => write!(f, "({expr}?)"),
      Self::Add(lhs, rhs) => write!(f, "({lhs} + {rhs})"),
      Self::Sub(lhs, rhs) => write!(f, "({lhs} - {rhs})"),
      Self::Mul(lhs, rhs) => write!(f, "({lhs} * {rhs})"),
      Self::Div(lhs, rhs) => write!(f, "({lhs} / {rhs})"),
      Self::Mod(lhs, rhs) => write!(f, "({lhs} % {rhs})"),
      Self::LogicalAnd(lhs, rhs) => write!(f, "({lhs} && {rhs})"),
      Self::LogicalOr(lhs, rhs) => write!(f, "({lhs} || {rhs})"),
      Self::And(lhs, rhs) => write!(f, "({lhs} & {rhs})"),
      Self::Or(lhs, rhs) => write!(f, "({lhs} | {rhs})"),
      Self::Xor(lhs, rhs) => write!(f, "({lhs} ^ {rhs})"),
      Self::Equal(lhs, rhs) => write!(f, "({lhs} == {rhs})"),
      Self::NotEqual(lhs, rhs) => write!(f, "({lhs} != {rhs})"),
      Self::Less(lhs, rhs) => write!(f, "({lhs} < {rhs})"),
      Self::Greater(lhs, rhs) => write!(f, "({lhs} > {rhs})"),
      Self::LessEqual(lhs, rhs) => write!(f, "({lhs} <= {rhs})"),
      Self::GreaterEqual(lhs, rhs) => write!(f, "({lhs} >= {rhs})"),
      Self::LeftShift(lhs, rhs) => write!(f, "({lhs} << {rhs})"),
      Self::RightShift(lhs, rhs) => write!(f, "({lhs} >> {rhs})"),
      Self::Cmp(lhs, rhs) => write!(f, "({lhs} <> {rhs})"),
      Self::ExclusiveRange(lhs, rhs) => write!(f, "({lhs} .. {rhs})"),
      Self::InclusiveRange(lhs, rhs) => write!(f, "({lhs} ... {rhs})"),
      Self::GetItem(lhs, rhs) => write!(f, "({lhs}::{rhs})"),
      Self::GetField(lhs, rhs) => write!(f, "({lhs}.{rhs})"),
    }
  }
}

/// Parse expressions using a binding power algorithm
fn parse_expr_bp(tokens: &mut Tokens, min_bp: u8) -> Expr {
  // left-hand side
  let mut lhs = match tokens.next() {
    Tk::Ident(ident) => Expr::Single(Value::Ident(ident.clone())),
    Tk::String(string) => Expr::Single(Value::String(string.clone())),
    Tk::FString(fstring) => Expr::Single(Value::FString(fstring.to_parsed())),
    Tk::Char(ch) => Expr::Single(Value::Char(*ch)),
    Tk::Number(number) => Expr::Single(Value::Number(number.clone())),
    Tk::LeftParen => {
      let lhs = parse_expr_bp(tokens, 0);
      match tokens.next() {
        Tk::RightParen => {}
        _ => syntax_err!("unclosed parenthesis")
      }
      lhs
    }
    Tk::Op(op) => {
      let right_bp = op.prefix_bp();
      let rhs = parse_expr_bp(tokens, right_bp);
      op.prefix_expr(rhs)
    }
    Tk::NewLine => return parse_expr_bp(tokens, min_bp),
    Tk::EoF => syntax_err!("expected value"),
    token => syntax_err!("unexpected {token}")
  };
  loop {
    let op = match &tokens.peek_amount(2)[0..2] {
      [None, None] => break,
      [Some(Tk::NewLine), Some(Tk::Op(op))] => {
        tokens.next();
        op
      },
      [Some(Tk::NewLine), Some(Tk::NewLine)] => {
        tokens.next();
        tokens.next();
        while let Tk::NewLine = tokens.peek() {
          tokens.next();
        }
        continue;
      }
      [Some(Tk::Op(op)), _] => op,
      // Usually [Some(NewLine), _]
      _ => return lhs
    };
    if let Some(left_bp) = op.postfix_bp() {
      if left_bp < min_bp {
        break;
      }
      tokens.next();
      lhs = op.postfix_expr(lhs);
      continue;
    }
    if let Some((left_bp, right_bp)) = op.infix_bp() {
      if left_bp < min_bp {
        break;
      }
      tokens.next();
      let rhs = parse_expr_bp(tokens, right_bp);
      lhs = op.infix_expr(lhs, rhs);
      continue;
    }
    break;
  }
  lhs
}
