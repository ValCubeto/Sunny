use std::fmt;
use super::CharsIter;
use super::tokens::Token;

// a!
// mod
// `...`
// $a
// $a: T[tk]+
// $a: T[tk]*
// f"{a:fmt}"

pub fn parse_word(chars: &mut CharsIter, ch: char) -> (Token, usize) {
  let mut word = String::from(ch);
  while let Some(ch) = chars.peek() {
    if word.len() > 30 {
      syntax_err!("this identifier is too long");
    }
    match ch {
      'a'..='z' => {
        word.push(ch);
        chars.next();
      }
      // Can't be a keyword, no need to check
      'A'..='Z' | '0'..='9' | '_' => {
        word.push(ch);
        chars.next();
        while let Some(ch @ ('a'..='z' | 'A'..='Z' | '0'..='9' | '_')) = chars.peek() {
          if word.len() > 30 {
            syntax_err!("this identifier is too long");
          }
          word.push(ch);
          chars.next();
        }
        
        if chars.peek() == Some('!') {
          chars.next();
          let len = word.len() + 1;
          return (Token::MacroName(word), len);
        }
        let len = word.len();
        return (Token::Ident(word), len);
      }
      '!' => {
        let len = word.len() + 1;
        return (Token::MacroName(word), len);
      }
      _ => break
    }
  }
  let len = word.len();
  let token = match Keyword::parse(&word) {
    Some(kw) => Token::Keyword(kw),
    None => Token::Ident(word)
  };
  (token, len)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
  /// `mod`
  Mod,

  /// `use`
  Use,
  /// `as`
  As,

  /// `shared`
  Shared,
  /// `hidden`
  Hidden,

  /// `const`
  Const,
  /// `state`
  State,
  /// `let`
  Let,
  /// `var`
  Var,

  /// `typedef`
  TypeDef,

  /// `struct`
  Struct,
  /// `enum`
  Enum,
  /// `bitset`
  BitSet,
  /// `interface`
  Interface,

  /// `class`
  Class,
  /// `where`
  Where,
  /// `impl`
  Impl,

  /// `arglist`
  ArgList,
  /// `fun`
  Fun,
  /// `takes`
  Takes,
  /// `defer`
  Defer,
  /// `return`
  Return,
  
  /// `unsafe`
  Unsafe,

  /// `async`
  Async,
  /// `await`
  Await,

  /// `loop`
  Loop,
  /// `while`
  While,
  /// `for`
  For,
  /// `in`
  In,
  /// `break`
  Break,
  /// `continue`
  Continue,

  /// `if`
  If,
  /// `else`
  Else,

  /// `match`
  Match,
  /// `is`
  Is,

  /// `macro`
  Macro,
  /// `case`
  Case,

  /// `Self`
  SelfType,
  /// `Super`
  SuperType,
  /// `never`
  Never
}

impl fmt::Display for Keyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(match self {
      Self::Takes => "takes",
      Self::Mod => "mod",
      Self::Use => "use",
      Self::As => "as",
      Self::Const => "const",
      Self::State => "state",
      Self::Let => "let",
      Self::Var => "var",
      Self::TypeDef => "typedef",
      Self::Struct => "struct",
      Self::Enum => "enum",
      Self::BitSet => "bitset",
      Self::Interface => "interface",
      Self::Class => "class",
      Self::Where => "where",
      Self::Impl => "impl",
      Self::ArgList => "arglist",
      Self::Fun => "fun",
      Self::Defer => "defer",
      Self::Return => "return",
      Self::Unsafe => "unsafe",
      Self::Async => "async",
      Self::Await => "await",
      Self::Loop => "loop",
      Self::While => "while",
      Self::For => "for",
      Self::In => "in",
      Self::Break => "break",
      Self::Continue => "continue",
      Self::If => "if",
      Self::Else => "else",
      Self::Match => "match",
      Self::Is => "is",
      Self::Macro => "macro",
      Self::Case => "case",
      Self::Shared => "shared",
      Self::Hidden => "hidden",
      Self::SelfType => "Self",
      Self::SuperType => "Super",
      Self::Never => "never",
    })
  }
}

impl Keyword {
  pub fn parse(input: &str) -> Option<Self> {
    let keyword = match input {
      "takes" => Self::Takes,
      "mod" => Self::Mod,
      "use" => Self::Use,
      "as" => Self::As,
      "const" => Self::Const,
      "state" => Self::State,
      "var" => Self::Var,
      "let" => Self::Let,
      "class" => Self::Class,
      "struct" => Self::Struct,
      "enum" => Self::Enum,
      "interface" => Self::Interface,
      "impl" => Self::Impl,
      "arglist" => Self::ArgList,
      "where" => Self::Where,
      "fun" => Self::Fun,
      "return" => Self::Return,
      "loop" => Self::Loop,
      "while" => Self::While,
      "for" => Self::For,
      "in" => Self::In,
      "break" => Self::Break,
      "continue" => Self::Continue,
      "if" => Self::If,
      "else" => Self::Else,
      "match" => Self::Match,
      "is" => Self::Is,
      "macro" => Self::Macro,
      "bitset" => Self::BitSet,
      "defer" => Self::Defer,
      "unsafe" => Self::Unsafe,
      "async" => Self::Async,
      "await" => Self::Await,
      "case" => Self::Case,
      "typedef" => Self::TypeDef,
      "shared" => Self::Shared,
      "hidden" => Self::Hidden,
      "Self" => Self::SelfType,
      "Super" => Self::SuperType,
      "never" => Self::Never,
      _ => return None,
    };
    Some(keyword)
  }
}
