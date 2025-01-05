use std::fmt;

#[allow(unused)]
#[derive(Debug)]
pub enum Keyword {
  Use,
  As,

  Shared,
  Hidden,

  Const,
  State,
  Let,
  Var,

  TypeDef,

  Struct,
  Enum,
  BitSet,
  Idea,

  Class,
  Extends,
  Where,
  Impl,

  ArgStruct,
  Fun,
  Defer,
  Return,

  Unsafe,

  Async,
  Await,

  Loop,
  While,
  For,
  In,
  Break,
  Continue,

  If,
  Else,

  Match,
  Is,

  Macro,
  Case,
}

impl fmt::Display for Keyword {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    debug_todo!("use some kind of sorted list");
    let word = match self {
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
      Self::Idea => "idea",
      Self::Class => "class",
      Self::Extends => "extends",
      Self::Where => "where",
      Self::Impl => "impl",
      Self::ArgStruct => "argstruct",
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
      Self::Hidden=> "hidden",
    };
    write!(f, "{word:?}")
  }
}

impl Keyword {
  pub fn parse(input: &str) -> Option<Self> {
    let keyword = match input {
      "use" => Self::Use,
      "as" => Self::As,
      "const" => Self::Const,
      "state" => Self::State,
      "var" => Self::Var,
      "let" => Self::Let,
      "class" => Self::Class,
      "struct" => Self::Struct,
      "enum" => Self::Enum,
      "idea" => Self::Idea,
      "impl" => Self::Impl,
      "argstruct" => Self::ArgStruct,
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
      "extends" => Self::Extends,
      "bitset" => Self::BitSet,
      "defer" => Self::Defer,
      "unsafe" => Self::Unsafe,
      "async" => Self::Async,
      "await" => Self::Await,
      "case" => Self::Case,
      "typedef" => Self::TypeDef,
      "shared" => Self::Shared,
      "hidden" => Self::Hidden,
      _ => return None,
    };
    Some(keyword)
  }
}
