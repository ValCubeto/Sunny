pub mod items;
pub mod constants;
pub mod expressions;
pub mod values;
pub mod types;
use constants::parse_static;
use items::Entity;
use peekmore::PeekMore;
use super::tokenize::tokens::{ Token as Tk, Tokens };
use super::tokenize::keywords::Keyword as Kw;
use super::tokenize::Position;

pub fn parse(tokens: Vec<(Position, Tk)>) -> Vec<Entity> {
  let mut items = Vec::new();
  let mut tokens = Tokens::new(tokens.iter().peekmore());
  while let Some(token) = tokens.next() {
    let item = match token {
      Tk::EoF => break,
      Tk::NewLine | Tk::Semicolon => continue,
      Tk::Keyword(Kw::Const) => parse_static(false, &mut tokens),
      Tk::Keyword(Kw::State) => parse_static(true, &mut tokens),
      Tk::Keyword(Kw::Hidden) => syntax_err!("not yet implemented"),
      Tk::Keyword(Kw::Shared) => syntax_err!("not yet implemented"),
      Tk::Keyword(Kw::Fun) => syntax_err!("functions not yet implemented"),
      Tk::Keyword(Kw::Use) => syntax_err!("imports not yet implemented"),
      Tk::Keyword(Kw::Struct) => syntax_err!("structs not yet implemented"),
      Tk::Keyword(Kw::Enum) => syntax_err!("enums not yet implemented"),
      Tk::Keyword(Kw::BitSet) => syntax_err!("bit sets not yet implemented"),
      Tk::Keyword(Kw::ArgStruct) => syntax_err!("argument structs not yet implemented"),
      Tk::Keyword(Kw::Class) => syntax_err!("classes not yet implemented"),
      Tk::Keyword(Kw::Idea) => syntax_err!("ideas not yet implemented"),
      Tk::Keyword(Kw::Impl) => syntax_err!("idea implementations not yet implemented"),
      Tk::Keyword(Kw::TypeDef) => syntax_err!("type definitions not yet implemented"),
      Tk::Keyword(Kw::Unsafe) => syntax_err!("unsafe functions not yet implemented"),
      Tk::Keyword(Kw::Async) => syntax_err!("async functions not yet implemented"),
      Tk::Keyword(Kw::If) => syntax_err!("static if statements not yet implemented"),
      Tk::Keyword(Kw::Macro) => syntax_err!("macros not yet implemented"),
      _ => syntax_err!("unexpected {token}"),
    };
    items.push(item);
  }
  items
}
