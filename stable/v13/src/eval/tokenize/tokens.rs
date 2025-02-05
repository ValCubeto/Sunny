use std::slice::Iter;
use std::fmt;
use std::sync::atomic::Ordering;
use peekmore::PeekMoreIterator;
use crate::eval::parse::expressions::Expr;
use crate::eval::parse::values::Value;
use super::keywords::Keyword;
use super::numbers::Number;
use super::strings::FString;
use super::{ Position, COLUMN, LINE, TOK_LEN };

type TokenIterator<'a> = PeekMoreIterator<Iter<'a, (Position, Token)>>;

pub struct TokenIter<'a>(TokenIterator<'a>);
impl<'a> TokenIter<'a> {
  pub fn new(tokens: TokenIterator<'a>) -> Self {
    TokenIter(tokens)
  }

  pub fn try_next_token(&mut self) -> Option<&'a Token> {
    self.skip_newline();
    self.0.next().map(|(pos, token)| {
      LINE.store(pos.line, Ordering::Relaxed);
      COLUMN.store(pos.column, Ordering::Relaxed);
      TOK_LEN.store(pos.tok_len, Ordering::Relaxed);
      token
    })
  }
  pub fn next_or(&mut self, msg: &str) -> &'a Token {
    match self.0.next() {
      None => syntax_err!("{msg}"),
      Some((pos, token)) => {
        LINE.store(pos.line, Ordering::Relaxed);
        COLUMN.store(pos.column, Ordering::Relaxed);
        TOK_LEN.store(pos.tok_len, Ordering::Relaxed);
        token
      }
    }
  }
  #[inline(always)]
  pub fn next(&mut self) -> &'a Token {
    self.next_or("unexpected end of input")
  }
  #[inline(always)]
  pub fn next_token_or(&mut self, msg: &str) -> &'a Token {
    self.skip_newline();
    self.next_or(msg)
  }
  #[inline(always)]
  pub fn next_token(&mut self) -> &'a Token {
    self.skip_newline();
    self.next()
  }

  #[inline(always)]
  fn peek_or(&mut self, msg: &str) -> &'a Token {
    match self.0.peek() {
      None => syntax_err!("{msg}"),
      Some((_, token)) => token
    }
  }
  #[inline(always)]
  pub fn peek(&mut self) -> &'a Token {
    self.peek_or("unexpected end of input")
  }
  #[inline(always)]
  pub fn peek_token(&mut self) -> &'a Token {
    self.skip_newline();
    self.peek()
  }

  #[inline(always)]
  pub fn skip_newline(&mut self) {
    if let Token::NewLine = self.peek() {
      self.next();
    }
  }

  pub fn comma_sep(&mut self) -> bool {
    match self.peek() {
      Token::Comma => {
        self.next();
        true
      }
      Token::NewLine => {
        self.next();
        if let Token::Comma = self.peek() {
          self.next();
        }
        true
      }
      _ => false
    }
  }

  pub fn semicolon_sep(&mut self) -> bool {
    match self.peek() {
      Token::Semicolon => {
        self.next();
        true
      }
      Token::NewLine => {
        self.next();
        if let Token::Semicolon = self.peek() {
          self.next();
        }
        true
      }
      _ => false
    }
  }
}

#[derive(Debug, Clone)]
/// # LEVEL 1
/// `a = b` <br>
/// `a += b` <br>
/// `a -= b` <br>
/// `a *= b` <br>
/// `a /= b` <br>
/// `a %= b` <br>
/// `a ^= b` <br>
/// `a &= b` <br>
/// `a |= b` <br>
/// `a &&= b` <br>
/// `a ||= b` <br>
/// `a <<= b` <br>
/// `a >>= b` <br>
/// # LEVEL 2
/// `a == b` <br>
/// `a != b` <br>
/// # LEVEL 3
/// `a && b` <br>
/// `a || b` <br>
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
  Caret,
  /// `&`
  Ampersand,
  /// `|`
  Pipe,
  /// `?`
  Question,
  /// `!`
  Bang,
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
  DoubleLeftAngle,
  /// `>>`
  DoubleRightAngle,
  /// `<=`
  LessOrEqual,
  /// `>=`
  GreaterOrEqual,
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
      Self::Plus | Self::Minus | Self::Bang | Self::Ampersand | Self::Star => 9,
      _ => syntax_err!("unexpected {}", self.to_string())
    }
  }
  pub fn infix_bp(&self) -> Option<(u8, u8)> {
    Some(match self {
      Self::Plus | Self::Minus => (5, 6),
      Self::Star | Self::Slash | Self::Percent => (7, 8),
      Self::Dot | Self::DoubleColon => (13, 14),
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
      _ => syntax_err!("unexpected {}", self.to_string())
    }
  }
  pub fn postfix_expr(&self, lhs: Expr) -> Expr {
    let lhs = lhs.ptr();
    match self {
      Self::Question => Expr::Try(lhs),
      _ => syntax_err!("unexpected {}", self.to_string())
    }
  }
  pub fn infix_expr(&self, lhs: Expr, rhs: Expr) -> Expr {
    let lhs = lhs.ptr();
    let rhs = rhs.ptr();
    match self {
      // `a + b`
      Self::Plus => Expr::Add(lhs, rhs),
      // `a - b`
      Self::Minus => Expr::Sub(lhs, rhs),
      // `a * b`
      Self::Star => Expr::Mul(lhs, rhs),
      // `a / b`
      Self::Slash => Expr::Div(lhs, rhs),
      // `a % b`
      Self::Percent => Expr::Mod(lhs, rhs),
      // `a <> b`
      Self::Diamond => Expr::Cmp(lhs, rhs),
      // `a && b`
      Self::DoubleAmpersand => Expr::LogicalAnd(lhs, rhs),
      // `a || b`
      Self::DoublePipe => Expr::LogicalOr(lhs, rhs),
      // `a & b`
      Self::Ampersand => Expr::And(lhs, rhs),
      // `a | b`
      Self::Pipe => Expr::Or(lhs, rhs),
      // `a ^ b`
      Self::Caret => Expr::Xor(lhs, rhs),
      // `a == b`
      Self::Equal => Expr::Equal(lhs, rhs),
      // `a != b`
      Self::NotEqual => Expr::NotEqual(lhs, rhs),
      // `a <= b`
      Self::LessOrEqual => Expr::LessEqual(lhs, rhs),
      // `a >= b`
      Self::GreaterOrEqual => Expr::GreaterEqual(lhs, rhs),
      // `a << b`
      Self::DoubleLeftAngle => Expr::LeftShift(lhs, rhs),
      // `a >> b`
      Self::DoubleRightAngle => Expr::RightShift(lhs, rhs),
      // `a.b`
      Self::Dot => match rhs.as_ref() {
        Expr::Ref(..) | Expr::Single(Value::String(..)) => Expr::GetField(lhs, rhs),
        _ => syntax_err!("field names must be strings or valid identifiers")
      }
      // `a::b`
      Self::DoubleColon => match rhs.as_ref() {
        Expr::Ref(..) | Expr::Single(Value::String(..)) => Expr::GetItem(lhs, rhs),
        _ => syntax_err!("item names must be strings or valid identifiers")
      }
      _ => syntax_err!("unexpected {}", self.to_string())
    }
  }
}

impl fmt::Display for Operator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Self::Plus => "plus sign",
      Self::Minus => "minus sign",
      Self::Star => "star",
      Self::Slash => "slash",
      Self::Percent => "percent",
      Self::Equal => "equal",
      Self::Caret => "caret",
      Self::Ampersand => "ampersand",
      Self::Pipe => "pipe",
      Self::Question => "question mark",
      Self::Bang => "bang",
      Self::TripleDot => "triple dot",
      Self::DoubleColon => "double colon",
      Self::DoubleEqual => "double equal",
      Self::NotEqual => "not equal",
      Self::LessOrEqual => "less equal",
      Self::GreaterOrEqual => "greater equal",
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
      Self::DoubleLeftAngle => "double left angle",
      Self::DoubleRightAngle => "double right angle",
      Self::LeftAngle => "left angle",
      Self::RightAngle => "right angle",
      Self::Dot => "dot",
      Self::Diamond => "diamond"
    })
  }
}

#[derive(Debug, Clone)]
/// I should create an Operator enum
pub enum Token {
  /// `if`, `fun`, `for`, etc
  Keyword(Keyword),
  /// Any valid variable name
  Ident(String),
  /// A variable name followed by a bang
  MacroName(String),
  Char(char),
  String(String),
  FString(FString),
  Number(Number),
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
  /// `{{`
  DoubleLeftBrace,
  /// `}}`
  DoubleRightBrace,
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
  /// `->`
  Arrow,
  /// `=>`
  FatArrow,
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Keyword(kw) => write!(f, "keyword {kw}"),
      Self::Ident(ident) => write!(f, "identifier {ident:?}"),
      Self::MacroName(name) => write!(f, "macro name {name:?}!"),
      Self::Op(op) => op.fmt(f),
      Self::String(..) => f.write_str("string literal"),
      Self::FString(..) => f.write_str("format string literal"),
      Self::Char(..) => f.write_str("char literal"),
      Self::Number(..) => f.write_str("number literal"),
      Self::NewLine => f.write_str("new line"),
      Self::LeftParen => f.write_str("left parenthesis"),
      Self::RightParen => f.write_str("right parenthesis"),
      Self::LeftBrace => f.write_str("left brace"),
      Self::RightBrace => f.write_str("right brace"),
      Self::DoubleLeftBrace => f.write_str("double left brace"),
      Self::DoubleRightBrace => f.write_str("double right brace"),
      Self::LeftBracket => f.write_str("left bracket"),
      Self::RightBracket => f.write_str("right bracket"),
      Self::FatArrow => f.write_str("fat arrow"),
      Self::Comma => f.write_str("comma"),
      Self::Semicolon => f.write_str("semicolon"),
      Self::Colon => f.write_str("colon"),
      Self::Arrow => f.write_str("arrow"),
    }
  }
}
