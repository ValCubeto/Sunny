pub mod tokenize;
pub mod parse;
use crate::ctx::Ctx;

pub fn eval(input: String, _ctx: Ctx) {
  let tokens = tokenize::tokenize(input);
  for (pos, token) in tokens.iter() {
    debug_msg!("({}, {}) Token[{}] {:?}", pos.line, pos.column, pos.tok_len, token);
  }
  debug_msg!("Tokenizing done");
  let entities = parse::parse(tokens);
  // debug!(items);
  for item in entities {
    debug_display!(item);
  }
  internal_err!("parsing done");
}
