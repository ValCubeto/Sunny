#[allow(unused)]
#[derive(Debug)]
pub enum Keyword {
  Use,
  As,
  From,

  Const,
  Var,
  Let,

  Class,
  Extends,
  Struct,
  Enum,
  BitSet,
  Idea,
  Impl,

  ArgStruct,
  Where,
  Fun,
  Return,

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
}

impl Keyword {
  // march parse(&word) { Some(k) => Keyword(k), None => Ident(word) }
  pub fn parse(input: &str) -> Option<Self> {
    use Keyword::*;
    let keyword = match input {
      "use" => Use,
      "as" => As,
      "from" => From,
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
      _ => return None,
    };
    Some(keyword)
  }
}
