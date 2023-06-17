use crate::context::Context;
use crate::errors::SyntaxError;
use crate::params::Params;

#[derive(Debug)]
pub enum Statment {
	#[allow(unused)]
	Call {
		name: String,
		params: Params
	}
}

#[allow(unused)]
#[derive(Debug)]
pub struct Function {
	pub name: String,
	// params: Params
	// returns: Type
	pub body: Vec<Statment>
}

pub fn parse_function(ctx: &mut Context) -> Function {
	let mut function = Function {
		name: String::new(),
		body: Vec::new()
	};
	while ctx.idx < ctx.chars.len() {
		ctx.next_char();
		match ctx.ch {
			'\n' => {
				ctx.idx += 1;
				ctx.line += 1;
				ctx.column = 1;
				continue;
			}
			' ' | '\t' | '\r' => {
				// ignore
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				function.name.push(ctx.ch);
				ctx.idx += 1;
				while ctx.idx < ctx.chars.len() {
					let ch = ctx.chars[ctx.idx];
					match ch {
						'a'..='z' | 'A'..='Z' | '_' => {
							function.name.push(ch);
						}
						_ => {
							break;
						}
					}
					ctx.idx += 1;
				}
				println!("parsing function {:?} at {}:{}:{}", function.name, ctx.id, ctx.line, ctx.column);
			}
			_ => {
				SyntaxError!("unexpected char {:?}", ctx.ch);
			}
		}
	}
	function
}
