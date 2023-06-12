use crate::context::Context;
use crate::errors::SyntaxError;

pub struct Function {}

pub fn parse_function(ctx: &mut Context) -> Function {
	let mut ch: char;

	while ctx.idx < ctx.chars.len() {
		ch = ctx.chars[ctx.idx];
		match ch {
			'\n' => {
				ctx.idx += 1;
				ctx.line += 1;
				ctx.column = 1;
				continue;
			}
			' ' => {
				// continue...
			}
			_ => {
				SyntaxError!("unexpected char {ch:?}");
			}
		}
		ctx.idx += 1;
		ctx.column += 1;
	}
	Function {}
}
