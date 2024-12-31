use std::iter::Peekable;
use std::slice::Iter;
use crate::eval::parse::expressions::Expr;
use std::fmt::{ Display, Formatter };
use super::{keywords::Keyword, number::Number};

pub type Tokens<'a> = Peekable<Iter<'a, Token>>;

#[allow(unused)]
#[derive(Debug)]
pub enum Token {
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
  /// `<`
  LeftAngle,
  /// `>`
  RightAngle,
  /// `[`
  LeftBracket,
  /// `]`
  RightBracket,
  /// `.`
  Dot,
  /// `,`
  Comma,
  /// `;`
  Semicolon,
  /// `:`
  Colon,
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
  /// `=>`
  Arrow,

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
  /// `**=`
  PowAssign,

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
      Token::NewLine => write!(f, "new line"),
      Token::LeftParen => write!(f, "left parenthesis"),
      Token::RightParen => write!(f, "right parenthesis"),
      Token::LeftBrace => write!(f, "left brace"),
      Token::RightBrace => write!(f, "right brace"),
      Token::LeftAngle => write!(f, "left angle"),
      Token::RightAngle => write!(f, "right angle"),
      Token::LeftBracket => write!(f, "left bracket"),
      Token::RightBracket => write!(f, "right bracket"),
      Token::Dot => write!(f, "dot"),
      Token::Comma => write!(f, "comma"),
      Token::Semicolon => write!(f, "semicolon"),
      Token::Colon => write!(f, "colon"),
      Token::Plus => write!(f, "plus"),
      Token::Minus => write!(f, "minus"),
      Token::Star => write!(f, "star"),
      Token::Slash => write!(f, "slash"),
      Token::Percent => write!(f, "percent"),
      Token::Equal => write!(f, "equal"),
      Token::Xor => write!(f, "xor"),
      Token::Ampersand => write!(f, "ampersand"),
      Token::Pipe => write!(f, "pipe"),
      Token::Question => write!(f, "question mark"),
      Token::Bang => write!(f, "bang"),
      Token::DoubleDot => write!(f, "double dot"),
      Token::TripleDot => write!(f, "triple dot"),
      Token::DoubleColon => write!(f, "double colon"),
      Token::DoubleEqual => write!(f, "double equal"),
      Token::NotEqual => write!(f, "not equal"),
      Token::Diamond => write!(f, "diamond"),
      Token::LeftShift => write!(f, "left shift"),
      Token::RightShift => write!(f, "right shift"),
      Token::LessEqual => write!(f, "less equal"),
      Token::GreaterEqual => write!(f, "greater equal"),
      Token::DoubleAmpersand => write!(f, "double ampersand"),
      Token::DoublePipe => write!(f, "double pipe"),
      Token::Arrow => write!(f, "arrow"),
      Token::AddAssign => write!(f, "add assign"),
      Token::SubAssign => write!(f, "sub assign"),
      Token::MulAssign => write!(f, "mul assign"),
      Token::DivAssign => write!(f, "div assign"),
      Token::ModAssign => write!(f, "mod assign"),
      Token::XorAssign => write!(f, "xor assign"),
      Token::AndAssign => write!(f, "and assign"),
      Token::OrAssign => write!(f, "or assign"),
      Token::LogicalAndAssign => write!(f, "logical and assign"),
      Token::LogicalOrAssign => write!(f, "logical or assign"),
      Token::LeftShiftAssign => write!(f, "left shift assign"),
      Token::RightShiftAssign => write!(f, "right shift assign"),
      Token::PowAssign => write!(f, "pow assign"),
      Token::Keyword(kw) => write!(f, "keyword {kw:?}"),
      Token::Ident(ident) => write!(f, "identifier {ident:?}"),
      Token::String(string) => write!(f, "{string:?}"),
      Token::Number(n) => write!(f, "number {n}"),
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
  #[inline]
  pub fn is_op(&self) -> bool {
    matches!(self,
      | Self::Plus
      | Self::Minus
      | Self::Star
      | Self::Slash
      | Self::Percent
      | Self::Diamond
      // | Self::Equal
      // | Self::DoubleDot
      // | Self::TripleDot
      // | Self::DoubleColon
      // | Self::DoubleEqual
      // | Self::NotEqual
      // | Self::LeftShift
      // | Self::RightShift
      // | Self::LessEqual
      // | Self::GreaterEqual
      // | Self::DoubleAmpersand
      // | Self::DoublePipe
      // | Self::AddAssign
      // | Self::SubAssign
      // | Self::MulAssign
      // | Self::DivAssign
      // | Self::ModAssign
      // | Self::XorAssign
      // | Self::AndAssign
      // | Self::OrAssign
      // | Self::LogicalAndAssign
      // | Self::LogicalOrAssign
      // | Self::LeftShiftAssign
      // | Self::RightShiftAssign
      // | Self::PowAssign
    )
  }
  pub fn prefix_expr(&self, rhs: Expr) -> Expr {
    let rhs = rhs.ptr();
    match self {
      Token::Plus => Expr::Pos(rhs),
      Token::Minus => Expr::Neg(rhs),
      Token::Bang => Expr::Not(rhs),
      Token::Ampersand => Expr::Ref(rhs),
      Token::Star => Expr::Deref(rhs),
      _ => syntax_err!("unexpected {self}")
    }
  }
  pub fn postfix_expr(&self, lhs: Expr) -> Expr {
    let lhs = lhs.ptr();
    match self {
      Token::Question => Expr::Try(lhs),
      _ => syntax_err!("unexpected {self}")
    }
  }
  pub fn infix_expr(&self, lhs: Expr, rhs: Expr) -> Expr {
    let lhs = lhs.ptr();
    let rhs = rhs.ptr();
    match self {
      Token::Plus => Expr::Add(lhs, rhs),
      Token::Minus => Expr::Sub(lhs, rhs),
      Token::Star => Expr::Mul(lhs, rhs),
      Token::Slash => Expr::Div(lhs, rhs),
      Token::Percent => Expr::Mod(lhs, rhs),
      Token::Diamond => Expr::Cmp(lhs, rhs),
      // Token::DoubleAmpersand => Expr::LogicalAnd(lhs, rhs),
      // Token::DoublePipe => Expr::LogicalOr(lhs, rhs),
      // Token::Ampersand => Expr::And(lhs, rhs),
      // Token::Pipe => Expr::Or(lhs, rhs),
      // Token::Xor => Expr::Xor(lhs, rhs),
      // Token::Equal => Expr::Equal(lhs, rhs),
      // Token::NotEqual => Expr::NotEqual(lhs, rhs),
      // Token::LeftAngle => Expr::Less(lhs, rhs),
      // Token::RightAngle => Expr::Greater(lhs, rhs),
      // Token::LessEqual => Expr::LessEqual(lhs, rhs),
      // Token::GreaterEqual => Expr::GreaterEqual(lhs, rhs),
      // Token::LeftShift => Expr::LeftShift(lhs, rhs),
      _ => syntax_err!("unexpected {self}")
    }
  }
}
