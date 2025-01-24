use std::slice::Iter;
use std::fmt;
use peekmore::PeekMoreIterator;
use crate::eval::parse::expressions::Expr;
use super::keywords::Keyword;
use super::numbers::Number;
use super::strings::FString;
use super::{ Position, COLUMN, LINE, TOK_LEN };

type TokenIterator<'a> = PeekMoreIterator<Iter<'a, (Position, Token)>>;

pub struct Tokens<'a>(TokenIterator<'a>);
impl<'a> Tokens<'a> {
  pub fn new(tokens: TokenIterator<'a>) -> Self {
    Tokens(tokens)
  }

  pub fn try_next_token(&mut self) -> Option<&'a Token> {
    self.skip_newline();
    self.0.next().map(|(pos, token)| -> &'a Token {
      unsafe {
        LINE = pos.line;
        COLUMN = pos.column;
        TOK_LEN = pos.tok_len;
      }
      token
    })
  }
  pub fn next_or(&mut self, msg: &str) -> &'a Token {
    match self.0.next() {
      None => syntax_err!("{msg}"),
      Some((pos, token)) => {
        unsafe {
          LINE = pos.line;
          COLUMN = pos.column;
          TOK_LEN = pos.tok_len;
        }
        token
      }
    }
  }
  #[inline(always)]
  pub fn next(&mut self) -> &'a Token {
    self.next_or("unexpected end of input")
  }
  pub fn next_token_or(&mut self, msg: &str) -> &'a Token {
    self.skip_newline();
    self.next_or(msg)
  }
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

  #[inline]
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

  #[inline]
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

#[allow(unused)]
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
      _ => syntax_err!("unexpected {self}")
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
      Self::DoubleColon => Expr::GetItem(lhs, rhs),
      Self::DoubleAmpersand => Expr::LogicalAnd(lhs, rhs),
      Self::DoublePipe => Expr::LogicalOr(lhs, rhs),
      Self::Ampersand => Expr::And(lhs, rhs),
      Self::Pipe => Expr::Or(lhs, rhs),
      Self::Caret => Expr::Xor(lhs, rhs),
      Self::Equal => Expr::Equal(lhs, rhs),
      Self::NotEqual => Expr::NotEqual(lhs, rhs),
      Self::LessOrEqual => Expr::LessEqual(lhs, rhs),
      Self::GreaterOrEqual => Expr::GreaterEqual(lhs, rhs),
      Self::DoubleLeftAngle => Expr::LeftShift(lhs, rhs),
      Self::DoubleRightAngle => Expr::RightShift(lhs, rhs),
      _ => syntax_err!("unexpected {self}")
    }
  }
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
      Self::Caret => "caret",
      Self::Ampersand => "ampersand",
      Self::Pipe => "pipe",
      Self::Question => "question mark",
      Self::Bang => "bang",
      Self::DoubleDot => "double dot",
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
      Self::Diamond => "diamond",
    };
    f.write_str(name)
  }
}

#[allow(unused)]
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
      Self::String(_) => write!(f, "string literal"),
      Self::FString(_) => write!(f, "format string literal"),
      Self::Char(_) => write!(f, "char literal"),
      Self::Number(_) => write!(f, "number literal"),
      Self::Op(op) => write!(f, "{op}"),
      Self::NewLine => write!(f, "new line"),
      Self::LeftParen => write!(f, "left parenthesis"),
      Self::RightParen => write!(f, "right parenthesis"),
      Self::LeftBrace => write!(f, "left brace"),
      Self::RightBrace => write!(f, "right brace"),
      Self::DoubleLeftBrace => write!(f, "double left brace"),
      Self::DoubleRightBrace => write!(f, "double right brace"),
      Self::LeftBracket => write!(f, "left bracket"),
      Self::RightBracket => write!(f, "right bracket"),
      Self::Comma => write!(f, "comma"),
      Self::Semicolon => write!(f, "semicolon"),
      Self::Colon => write!(f, "colon"),
      Self::Arrow => write!(f, "arrow"),
      Self::FatArrow => write!(f, "fat arrow")
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
