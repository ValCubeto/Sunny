use std::iter::Peekable;
use std::slice::Iter;
use crate::eval::parse::expressions::Expr;
use std::fmt::{ Display, Formatter };
use super::{keywords::Keyword, number::Number};

pub type Tokens<'a> = Peekable<Iter<'a, Token>>;

#[allow(unused)]
#[derive(Debug)]
pub enum Operator {
  /// `<`
  LeftAngle,
  /// `>`
  RightAngle,
  /// `.`
  Dot,
  /// `+`
  Plus,
  /// `-`
  Minus,
  /// `*`
  Star,
  /// `/`
  Slash,
  /// `%`
  Percent,
  /// `=`
  Equal,
  /// `^`
  Xor,
  /// `&`
  Ampersand,
  /// `|`
  Pipe,
  /// `?`
  Question,
  /// `!`
  Bang,
  /// `..`
  DoubleDot,
  /// `...`
  TripleDot,
  /// `::`
  DoubleColon,
  /// `==`
  DoubleEqual,
  /// `!=`
  NotEqual,
  /// `<>`
  Diamond,
  /// `<<`
  LeftShift,
  /// `>>`
  RightShift,
  /// `<=`
  LessEqual,
  /// `>=`
  GreaterEqual,
  /// `&&`
  DoubleAmpersand,
  /// `||`
  DoublePipe,
  /// `+=`
  AddAssign,
  /// `-=`
  SubAssign,
  /// `*=`
  MulAssign,
  /// `/=`
  DivAssign,
  /// `%=`
  ModAssign,
  /// `^=`
  XorAssign,
  /// `&=`
  AndAssign,
  /// `|=`
  OrAssign,
  /// `&&=`
  LogicalAndAssign,
  /// `||=`
  LogicalOrAssign,
  /// `<<=`
  LeftShiftAssign,
  /// `>>=`
  RightShiftAssign,
}

impl Display for Operator {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Operator::Plus => write!(f, "plus sign"),
      Operator::Minus => write!(f, "minus sign"),
      Operator::Star => write!(f, "star"),
      Operator::Slash => write!(f, "slash"),
      Operator::Percent => write!(f, "percent"),
      Operator::Equal => write!(f, "equal"),
      Operator::Xor => write!(f, "xor"),
      Operator::Ampersand => write!(f, "ampersand"),
      Operator::Pipe => write!(f, "pipe"),
      Operator::Question => write!(f, "question mark"),
      Operator::Bang => write!(f, "bang"),
      Operator::DoubleDot => write!(f, "double dot"),
      Operator::TripleDot => write!(f, "triple dot"),
      Operator::DoubleColon => write!(f, "double colon"),
      Operator::DoubleEqual => write!(f, "double equal"),
      Operator::NotEqual => write!(f, "not equal"),
      Operator::LessEqual => write!(f, "less equal"),
      Operator::GreaterEqual => write!(f, "greater equal"),
      Operator::DoubleAmpersand => write!(f, "double ampersand"),
      Operator::DoublePipe => write!(f, "double pipe"),
      Operator::AddAssign => write!(f, "add-assign operator"),
      Operator::SubAssign => write!(f, "sub-assign operator"),
      Operator::MulAssign => write!(f, "mul-assign operator"),
      Operator::DivAssign => write!(f, "div-assign operator"),
      Operator::ModAssign => write!(f, "mod-assign operator"),
      Operator::XorAssign => write!(f, "xor-assign operator"),
      Operator::AndAssign => write!(f, "and-assign operator"),
      Operator::OrAssign => write!(f, "or-assign operator"),
      Operator::LogicalAndAssign => write!(f, "logical-and-assign operator"),
      Operator::LogicalOrAssign => write!(f, "logical-or-assign operator"),
      Operator::LeftShiftAssign => write!(f, "left-shift-assign operator"),
      Operator::RightShiftAssign => write!(f, "right-shift-assign operator"),
      Operator::LeftShift => write!(f, "left shift"),
      Operator::RightShift => write!(f, "right shift"),
      Operator::LeftAngle => write!(f, "left angle"),
      Operator::RightAngle => write!(f, "right angle"),
      Operator::Dot => write!(f, "dot"),
      Operator::Diamond => write!(f, "diamond"),
    }
  }
}

#[allow(unused)]
#[derive(Debug)]
/// I should create an Operator enum
pub enum Token {
  /// Any operator
  Op(Operator),
  /// `\n`, `\r\n`
  NewLine,
  /// `(`
  LeftParen,
  /// `)`
  RightParen,
  /// `{`
  LeftBrace,
  /// `}`
  RightBrace,
  /// `[`
  LeftBracket,
  /// `]`
  RightBracket,
  /// `,`
  Comma,
  /// `;`
  Semicolon,
  /// `:`
  Colon,
  /// `=>`
  Arrow,
  /// `if`, `fun`, `for`, etc
  Keyword(Keyword),
  /// Any valid variable name
  Ident(String),
  String(String),
  Number(Number),
}

impl Display for Token {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Token::Keyword(kw) => write!(f, "keyword {kw:?}"),
      Token::Ident(ident) => write!(f, "identifier {ident:?}"),
      Token::String(string) => write!(f, "string {string:?}"),
      Token::Number(n) => write!(f, "number {n}"),
      Token::Op(op) => write!(f, "{op}"),
      Token::NewLine => write!(f, "new line"),
      Token::LeftParen => write!(f, "left parenthesis"),
      Token::RightParen => write!(f, "right parenthesis"),
      Token::LeftBrace => write!(f, "left brace"),
      Token::RightBrace => write!(f, "right brace"),
      Token::LeftBracket => write!(f, "left bracket"),
      Token::RightBracket => write!(f, "right bracket"),
      Token::Comma => write!(f, "comma"),
      Token::Semicolon => write!(f, "semicolon"),
      Token::Colon => write!(f, "colon"),
      Token::Arrow => write!(f, "arrow"),
      // _ => unimplemented!()
    }
  }
}

#[allow(unused)]
impl Token {
  #[inline]
  pub fn ptr(self) -> Box<Token> {
    Box::new(self)
  }
}

// It's an inferno to manually write all binding powers
impl Operator {
  // They usually use dummies
  /// `Option<(u8, ())>`
  pub fn postfix_bp(&self) -> Option<u8> {
    match self {
      | Self::Question => Some(11),
      _ => None
    }
  }
  /// `Option<((), u8)>`
  pub fn prefix_bp(&self) -> u8 {
    match self {
      Self::Plus | Self::Minus => 9,
      _ => syntax_err!("unexpected {self}")
    }
  }
  pub fn infix_bp(&self) -> Option<(u8, u8)> {
    Some(match self {
      Self::Equal => (2, 1),
      Self::Plus | Self::Minus => (5, 6),
      Self::Star | Self::Slash | Self::Percent => (7, 8),
      Self::Dot => (14, 13),
      _ => return None
    })
  }

  pub fn prefix_expr(&self, rhs: Expr) -> Expr {
    let rhs = rhs.ptr();
    match self {
      Self::Plus => Expr::Pos(rhs),
      Self::Minus => Expr::Neg(rhs),
      Self::Bang => Expr::Not(rhs),
      Self::Ampersand => Expr::Ref(rhs),
      Self::Star => Expr::Deref(rhs),
      _ => syntax_err!("unexpected {self}")
    }
  }
  pub fn postfix_expr(&self, lhs: Expr) -> Expr {
    let lhs = lhs.ptr();
    match self {
      Self::Question => Expr::Try(lhs),
      _ => syntax_err!("unexpected {self}")
    }
  }
  pub fn infix_expr(&self, lhs: Expr, rhs: Expr) -> Expr {
    let lhs = lhs.ptr();
    let rhs = rhs.ptr();
    match self {
      Self::Plus => Expr::Add(lhs, rhs),
      Self::Minus => Expr::Sub(lhs, rhs),
      Self::Star => Expr::Mul(lhs, rhs),
      Self::Slash => Expr::Div(lhs, rhs),
      Self::Percent => Expr::Mod(lhs, rhs),
      Self::Diamond => Expr::Cmp(lhs, rhs),
      Self::Dot => Expr::GetField(lhs, rhs),
      Self::DoubleAmpersand => Expr::LogicalAnd(lhs, rhs),
      Self::DoublePipe => Expr::LogicalOr(lhs, rhs),
      Self::Ampersand => Expr::And(lhs, rhs),
      Self::Pipe => Expr::Or(lhs, rhs),
      Self::Xor => Expr::Xor(lhs, rhs),
      Self::Equal => Expr::Equal(lhs, rhs),
      Self::NotEqual => Expr::NotEqual(lhs, rhs),
      Self::LessEqual => Expr::LessEqual(lhs, rhs),
      Self::GreaterEqual => Expr::GreaterEqual(lhs, rhs),
      Self::LeftShift => Expr::LeftShift(lhs, rhs),
      _ => syntax_err!("unexpected {self}")
    }
  }
}
