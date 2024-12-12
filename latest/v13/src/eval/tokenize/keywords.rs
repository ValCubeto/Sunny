#[allow(unused)]
#[derive(Debug)]
pub enum Keyword {
  Use,
  As,

  Public,
  Private,

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

impl Keyword {
  // march parse(&word) { Some(k) => Keyword(k), None => Ident(word) }
  pub fn parse(input: &str) -> Option<Self> {
    use Keyword::*;
    let keyword = match input {
      "use" => Use,
      "as" => As,
      "const" => Const,
      "var" => Var,
      "let" => Let,
      "class" => Class,
      "struct" => Struct,
      "enum" => Enum,
      "idea" => Idea,
      "impl" => Impl,
      "argstruct" => ArgStruct,
      "where" => Where,
      "fun" => Fun,
      "return" => Return,
      "loop" => Loop,
      "while" => While,
      "for" => For,
      "in" => In,
      "break" => Break,
      "continue" => Continue,
      "if" => If,
      "else" => Else,
      "match" => Match,
      "is" => Is,
      "macro" => Macro,
      "extends" => Extends,
      "bitset" => BitSet,
      "defer" => Defer,
      "unsafe" => Unsafe,
      "async" => Async,
      "await" => Await,
      "case" => Case,
      "pub" => Public,
      "priv" => Private,
      "typedef" => TypeDef,
      _ => return None,
    };
    Some(keyword)
  }
}
