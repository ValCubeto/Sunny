use super::values::Value;

#[derive(Debug)]
pub enum Expr {
  Single(Value),
  Add,
  Sub,
  Mul,
  Div,
  Mod,
  Pow,

  If,
  For,

  Let,
  Var,
}
