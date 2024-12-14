use std::iter::Peekable;
use std::slice::Iter;
use super::keywords::Keyword;

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
  And,
  /// `|`
  Or,
  /// `?`
  Question,
  /// `!`
  Bang,

  /// `**`
  DoubleStar,
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
  LogicalAnd,
  /// `||`
  LogicalOr,
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

  /// `0`-`9`
  Int(String),
  /// `0`-`9` + `.` + `0`-`9`
  Float(String),
  /// `0x` + (`0`-`9` | `a`-`f` | `A`-`F`)
  HexNumber(String),
  /// `0b` + `0`-`1`
  BinNumber(String),
}
impl Token {
  #[inline]
  pub fn ptr(self) -> Box<Token> {
    Box::new(self)
  }
}
