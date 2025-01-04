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
    let name = match self {
      Self::Plus => "plus sign",
      Self::Minus => "minus sign",
      Self::Star => "star",
      Self::Slash => "slash",
      Self::Percent => "percent",
      Self::Equal => "equal",
      Self::Xor => "xor",
      Self::Ampersand => "ampersand",
      Self::Pipe => "pipe",
      Self::Question => "question mark",
      Self::Bang => "bang",
      Self::DoubleDot => "double dot",
      Self::TripleDot => "triple dot",
      Self::DoubleColon => "double colon",
      Self::DoubleEqual => "double equal",
      Self::NotEqual => "not equal",
      Self::LessEqual => "less equal",
      Self::GreaterEqual => "greater equal",
      Self::DoubleAmpersand => "double ampersand",
      Self::DoublePipe => "double pipe",
      Self::AddAssign => "add-assign operator",
      Self::SubAssign => "sub-assign operator",
      Self::MulAssign => "mul-assign operator",
      Self::DivAssign => "div-assign operator",
      Self::ModAssign => "mod-assign operator",
      Self::XorAssign => "xor-assign operator",
      Self::AndAssign => "and-assign operator",
      Self::OrAssign => "or-assign operator",
      Self::LogicalAndAssign => "logical-and-assign operator",
      Self::LogicalOrAssign => "logical-or-assign operator",
      Self::LeftShiftAssign => "left-shift-assign operator",
      Self::RightShiftAssign => "right-shift-assign operator",
      Self::LeftShift => "left shift",
      Self::RightShift => "right shift",
      Self::LeftAngle => "left angle",
      Self::RightAngle => "right angle",
      Self::Dot => "dot",
      Self::Diamond => "diamond",
    };
    f.write_str(name)
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
      Token::Keyword(kw) => write!(f, "keyword {kw}"),
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
      Self::Question => Some(11),
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
      // Self::Equal => (2, 1),
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
