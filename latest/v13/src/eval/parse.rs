pub mod items;
pub mod constants;
pub mod expressions;
pub mod values;
pub mod types;
pub mod functions;
use constants::parse_static;
use functions::parse_function;
use items::{ Entity, Metadata };
use peekmore::PeekMore;
use crate::eval::tokenize::{
  tokens::{ Token as Tk, Tokens },
  keywords::Keyword as Kw,
  Position
};

pub fn parse(tokens: Vec<(Position, Tk)>) -> Vec<Entity> {
  let mut items = Vec::new();
  let mut tokens = Tokens::new(tokens.iter().peekmore());
  let mut common_meta = Metadata::default();
  while let Some(token) = tokens.next() {
    if let Tk::EoF = token {
      break;
    }
    if let Tk::NewLine | Tk::Semicolon = token {
      continue;
    }
    if let Some(item) = match_token(&mut common_meta, token, &mut tokens) {
      items.push(item);
    }
  }
  items
}

fn match_token(metadata: &mut Metadata, token: &Tk, tokens: &mut Tokens) -> Option<Entity> {
  Some(match token {
    Tk::Keyword(Kw::All) => {
      match tokens.next() {
        Some(Tk::Keyword(vis @ (Kw::Hidden | Kw::Shared))) => {
          metadata.hidden = vis == &Kw::Hidden;
        }
        Some(other) => syntax_err!("unexpected {other}"),
        None => syntax_err!("expected visibility modifier")
      }
      metadata.mutable = true;
      return None;
    }
    Tk::Keyword(Kw::Const) => {
      metadata.mutable = false;
      parse_static(*metadata, tokens)
    }
    Tk::Keyword(Kw::State) => {
      metadata.mutable = true;
      parse_static(*metadata, tokens)
    }
    Tk::Keyword(Kw::Hidden) => {
      tokens.skip_newline();
      if let Some(Tk::Keyword(Kw::Shared | Kw::Hidden)) = tokens.peek() {
        tokens.next();
        syntax_err!("multiple visibility modifiers");
      }
      let mut metadata = *metadata;
      metadata.hidden = true;
      match_token(&mut metadata, token, tokens)?
    }
    Tk::Keyword(Kw::Shared) => {
      tokens.skip_newline();
      if let Some(Tk::Keyword(Kw::Shared | Kw::Hidden)) = tokens.peek() {
        tokens.next();
        syntax_err!("multiple visibility modifiers");
      }
      let mut metadata = *metadata;
      metadata.hidden = false;
      match_token(&mut metadata, token, tokens)?
    }
    Tk::Keyword(Kw::Unsafe) => {
      if let Some(Tk::Keyword(Kw::Unsafe)) = tokens.peek() {
        tokens.next();
        syntax_err!("repeated unsafe keyword");
      }
      let mut metadata = *metadata;
      metadata.is_unsafe = true;
      match_token(&mut metadata, token, tokens)?
    }
    Tk::Keyword(Kw::Async) => {
      if let Some(Tk::Keyword(Kw::Async)) = tokens.peek() {
        tokens.next();
        syntax_err!("repeated async keyword");
      }
      // create a copy
      let mut metadata = *metadata;
      metadata.is_async = true;
      match_token(&mut metadata, token, tokens)?
    }
    Tk::Keyword(Kw::Fun) => {
      let name: String = match tokens.next() {
        Some(Tk::Ident(name)) => name.clone(),
        _ => syntax_err!("expected function name")
      };
      parse_function(*metadata, tokens, name)
    },
    Tk::Keyword(Kw::Use) => syntax_err!("imports not yet implemented"),
    Tk::Keyword(Kw::Struct) => syntax_err!("structs not yet implemented"),
    Tk::Keyword(Kw::Enum) => syntax_err!("enums not yet implemented"),
    Tk::Keyword(Kw::BitSet) => syntax_err!("bit sets not yet implemented"),
    Tk::Keyword(Kw::ArgStruct) => syntax_err!("argument structs not yet implemented"),
    Tk::Keyword(Kw::Class) => syntax_err!("classes not yet implemented"),
    Tk::Keyword(Kw::Idea) => syntax_err!("ideas not yet implemented"),
    Tk::Keyword(Kw::Impl) => syntax_err!("idea implementations not yet implemented"),
    Tk::Keyword(Kw::TypeDef) => syntax_err!("type definitions not yet implemented"),
    Tk::Keyword(Kw::If) => syntax_err!("static if statements not yet implemented"),
    Tk::Keyword(Kw::Macro) => syntax_err!("macros not yet implemented"),
    _ => syntax_err!("unexpected {token}"),
  })
}
