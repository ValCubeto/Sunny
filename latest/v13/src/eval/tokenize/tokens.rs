use super::keywords::Keyword;

#[allow(unused)]
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
  /// `&`
  And,
  /// `|`
  Or,
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
  /// `<=`
  LessEqual,
  /// `>=`
  GreaterEqual,
  /// `&&`
  AndAnd,
  /// `||`
  OrOr,
  /// `=>`
  Arrow,

  /// `if`, `fun`, `for`, etc
  Keyword(Keyword),
  /// Any valid variable name
  Ident(String),

  /// `0`-`9`
  Int(String),
  /// `0`-`9` + `.` + `0`-`9`
  Float(String),
  /// `0x` + (`0`-`9` | `a`-`f` | `A`-`F`)
  HexNumber(String),
  /// `0b` + `0`-`1`
  BinNumber(String),
}
