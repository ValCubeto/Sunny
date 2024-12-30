use crate::eval::tokenize::tokens::{ Token, Tokens };
use super::constants::Variable;
use super::values::Value;

type E = Box<Expr>;

#[allow(unused)]
#[derive(Debug)]
pub enum Expr {
  Single(Value),

  /// `!a`
  Not(E),
  /// `-a`
  Neg(E),
  /// `*a`
  Deref(E),
  /// `&a`
  Ref(E),
  /// `a%`
  Percent(E),
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
  /// `a ** b`
  Pow(E, E),
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

  /// `a .. b`
  ExclusiveRange(E, E),
  /// `a ... b`
  InclusiveRange(E, E),

  /// `a = b`
  Assign(E, E),
  /// `a += b`
  AddAssign(E, E),
  /// `a -= b`
  SubAssign(E, E),
  /// `a *= b`
  MulAssign(E, E),
  /// `a /= b`
  DivAssign(E, E),
  /// `a %= b`
  ModAssign(E, E),
  /// `a **= b`
  PowAssign(E, E),
  /// `a &&= b`
  LogicalAndAssign(E, E),
  /// `a &= b`
  AndAssign(E, E),
  /// `a ||= b`
  LogicalOrAssign(E, E),
  /// `a |= b`
  OrAssign(E, E),
  /// `a ^= b`
  XorAssign(E, E),

  /// `a :: b`
  GetItem(E, String),
  /// `a . b`
  GetProp(E, String),

  // Call(ident, generics, args),
  Let(Box<Variable>),
  Var(Box<Variable>),

  Loop(Vec<Expr>),
  While(E, Vec<Expr>),
  If(E, Vec<Expr>, Vec<Expr>),
  ForIn(E, E, Vec<Expr>),
}

impl Expr {
  pub fn ptr(self) -> Box<Expr> {
    Box::new(self)
  }
}

/*
in = [Num1, Plus, Num2, Star, Num3, Plus, Num4]
out = Sum(Sum(Num1, Mul(Num2, Num3)), Num4)


var left_side = []
var cand = {
  left: tokens[0],
  op: tokens[1],
  right: tokens[2],
}
## cand = { left: Num1, op: Plus, right: Num2 }

* encontrar el primero con alta prioridad, separando la lista donde este se encuentre
var token = tokens.next()
while token.prec() < cand.op.prec() {
  left_side.push(cands.left, cands.op)
  cand.left = cands.right
  cand.op = token
  token = tokens.next()
  cand.right = token
}
## left_side = [Num1, Plus]
## cand = { left: Num2, op: Star, right: Num3 }

* crear una expresion con el candidato
let expr = make_expr(cand)
## expr = Mul(Num2, Num3)

* encontrar el siguiente con alta prioridad
while left_side.next().prec() < expr.prec() {
*/

/// None if there are no tokens
pub fn parse_expr(tokens: &mut Tokens) -> Option<Expr> {
  todo!("list and sort tokens");
  let token = tokens.next()?;
  parse_expr_(expect_value(token), tokens)
}

fn parse_expr_(prev: Expr, tokens: &mut Tokens) -> Option<Expr> {
  match tokens.next()? {
    Token::Plus => parse_expr_(Expr::Add(prev.ptr(), expect_value(tokens.next()?).ptr()), tokens),
    Token::Minus => parse_expr_(Expr::Sub(prev.ptr(), expect_value(tokens.next()?).ptr()), tokens),
    Token::Star => parse_expr_(Expr::Mul(prev.ptr(), expect_value(tokens.next()?).ptr()), tokens),
    Token::Slash => parse_expr_(Expr::Div(prev.ptr(), expect_value(tokens.next()?).ptr()), tokens),
    Token::DoubleStar => parse_expr_(Expr::Pow(prev.ptr(), expect_value(tokens.next()?).ptr()), tokens),
    Token::NewLine => todo!(),
    _ => Some(prev),
  }
}

fn expect_value(value: &Token) -> Expr {
  Expr::Single(match value {
    Token::String(string) => Value::String(string.clone()),
    // Token::Int(_) => ,
    // Token::Float(_) => ,
    _ => syntax_err!("expected value"),
  })
}
