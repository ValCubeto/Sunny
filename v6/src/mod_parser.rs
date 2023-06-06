use crate::context::Context;
use crate::errors::SyntaxError;
use crate::func_parser::parse_function;

pub struct Module {}

impl Module {
	// pub fn exec_function(name: String, arguments: Arguments) {}
}

pub fn parse_module(code: String, id: String) -> Module {
	let mut ch: char;

	let ctx = &mut Context {
		chars: code.chars().collect(),
		idx: 0,
		line: 1,
		column: 1
	};

	while ctx.idx < ctx.chars.len() {
		ch = ctx.chars[ctx.idx];
		match ch {
			'\n' => {
				ctx.line += 1;
				ctx.column = 1;
				continue;
			}
			' ' | '\t' | '\r' => {
				// continue...
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				let mut word = String::from(ch);
				ctx.idx += 1;
				// collect word
				while ctx.idx < ctx.chars.len() {
					ch = ctx.chars[ctx.idx];
					match ch {
						'a'..='z' | 'A'..='Z' | '_' => {
							ctx.column += 1;
							word.push(ch);
						}
						_ => {
							ctx.idx -= 1;
							break;
						}
					}
					ctx.idx += 1;
				}
				match word.as_str() {
					"fun" => {
						ctx.idx += 1;
						parse_function(ctx);
					}
					_ => {
						SyntaxError!("unexpected identifier {word:?} here")
					}
				}
			}
			_ => {
				SyntaxError!("unknown or unexpected char {ch:?}\n    at {id}:{}:{}", ctx.line, ctx.column);
			}
		}
		ctx.idx += 1;
		ctx.column += 1;
	}

	Module {}
}