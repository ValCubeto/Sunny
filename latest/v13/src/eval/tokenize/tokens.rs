use std::slice::Iter;
use std::fmt;
use peekmore::PeekMoreIterator;
use crate::eval::parse::expressions::Expr;
use super::{ keywords::Keyword, number::Number };

pub type Tokens<'a> = PeekMoreIterator<Iter<'a, Token>>;

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

impl fmt::Display for Operator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Plus => write!(f, "plus sign"),
      Self::Minus => write!(f, "minus sign"),
      Self::Star => write!(f, "star"),
      Self::Slash => write!(f, "slash"),
      Self::Percent => write!(f, "percent"),
      Self::Equal => write!(f, "equal"),
      Self::Xor => write!(f, "xor"),
      Self::Ampersand => write!(f, "ampersand"),
      Self::Pipe => write!(f, "pipe"),
      Self::Question => write!(f, "question mark"),
      Self::Bang => write!(f, "bang"),
      Self::DoubleDot => write!(f, "double dot"),
      Self::TripleDot => write!(f, "triple dot"),
      Self::DoubleColon => write!(f, "double colon"),
      Self::DoubleEqual => write!(f, "double equal"),
      Self::NotEqual => write!(f, "not equal"),
      Self::LessEqual => write!(f, "less equal"),
      Self::GreaterEqual => write!(f, "greater equal"),
      Self::DoubleAmpersand => write!(f, "double ampersand"),
      Self::DoublePipe => write!(f, "double pipe"),
      Self::AddAssign => write!(f, "add-assign operator"),
      Self::SubAssign => write!(f, "sub-assign operator"),
      Self::MulAssign => write!(f, "mul-assign operator"),
      Self::DivAssign => write!(f, "div-assign operator"),
      Self::ModAssign => write!(f, "mod-assign operator"),
      Self::XorAssign => write!(f, "xor-assign operator"),
      Self::AndAssign => write!(f, "and-assign operator"),
      Self::OrAssign => write!(f, "or-assign operator"),
      Self::LogicalAndAssign => write!(f, "logical-and-assign operator"),
      Self::LogicalOrAssign => write!(f, "logical-or-assign operator"),
      Self::LeftShiftAssign => write!(f, "left-shift-assign operator"),
      Self::RightShiftAssign => write!(f, "right-shift-assign operator"),
      Self::LeftShift => write!(f, "left shift"),
      Self::RightShift => write!(f, "right shift"),
      Self::LeftAngle => write!(f, "left angle"),
      Self::RightAngle => write!(f, "right angle"),
      Self::Dot => write!(f, "dot"),
      Self::Diamond => write!(f, "diamond"),
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

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    #[allow(unreachable_patterns, reason="Compile anyways even if I add more Token variants")]
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
      _ => unimplemented!()
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
      Self::Question | Self::Percent => Some(11),
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
      Self::Percent => Expr::Percent(lhs),
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
