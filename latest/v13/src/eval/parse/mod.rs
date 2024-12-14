pub mod items;
pub mod constants;
pub mod expressions;
pub mod values;
pub mod types;
use constants::parse_static;
use items::Entity;
use super::tokenize::tokens::Token as Tk;
use super::tokenize::keywords::Keyword as Kw;

pub fn parse(tokens: Vec<Tk>) -> Vec<Entity> {
  let mut items = Vec::new();
  let mut tokens = tokens.iter().peekable();
  while let Some(token) = tokens.next() {
    let item = match token {
      Tk::NewLine | Tk::Semicolon => continue,
      Tk::Keyword(Kw::Const) => parse_static(false, &mut tokens),
      Tk::Keyword(Kw::Private) => todo!(),
      Tk::Keyword(Kw::Public) => todo!(),
      Tk::Keyword(Kw::State) => todo!(),
      Tk::Keyword(Kw::Use) => todo!(),
      Tk::Keyword(Kw::Struct) => todo!(),
      Tk::Keyword(Kw::Enum) => todo!(),
      Tk::Keyword(Kw::BitSet) => todo!(),
      Tk::Keyword(Kw::Fun) => todo!(),
      Tk::Keyword(Kw::ArgStruct) => todo!(),
      Tk::Keyword(Kw::Class) => todo!(),
      Tk::Keyword(Kw::Idea) => todo!(),
      Tk::Keyword(Kw::Impl) => todo!(),
      Tk::Keyword(Kw::TypeDef) => todo!(),
      Tk::Keyword(Kw::Unsafe) => todo!(),
      Tk::Keyword(Kw::Async) => todo!(),
      Tk::Keyword(Kw::If) => todo!(),
      Tk::Keyword(Kw::Macro) => todo!(),
      Tk::Keyword(keyword) => syntax_err!("unexpected keyword \"{keyword}\" here"),
      Tk::Ident(ident) => syntax_err!("unexpected identifier \"{ident}\" here"),
      _ => syntax_err!("unexpected token {token:?} here"),
    };
    items.push(item);
  }
  debug!(items);
  items
}
