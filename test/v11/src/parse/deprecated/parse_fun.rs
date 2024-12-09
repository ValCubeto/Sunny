use super::Parser;

// PARAMS: FUNCTION
// `let sum = (a, b): a + b`
// ARGS: CALLER
// `let res = sum(1, 2)`

/// Parses a function after finding the `fun` keyword
pub fn parse_function(parser: &mut Parser) /* -> Function  */ {
  let ident = parser.expect_word();
}

pub fn parse_anon_function(parser: &mut Parser) /* -> Function */ {
}
