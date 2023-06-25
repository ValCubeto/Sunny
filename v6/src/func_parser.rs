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
		println!("{ctx:#}");
		match ctx.ch {
			' ' | '\t' | '\n' | '\r' => {
				// ignore
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				function.name.push(ctx.ch);
				while ctx.idx < ctx.chars.len() {
					ctx.next_char();
					println!("{ctx:#}");
					match ctx.ch {
						'a'..='z' | 'A'..='Z' | '_' => {
							function.name.push(ctx.ch);
						}
						' ' | '\n' | '\r' | '\t' => {
							// skip
						}
						'(' => {
							SyntaxError!(ctx, "todo");
						}
						_ => {
							SyntaxError!(ctx, "expected '('");
						}
					}
				}
				println!("parsing function {:?} at {}:{}:{}", function.name, ctx.id, ctx.line, ctx.column);
			}
			_ => {
				SyntaxError!(ctx, "unknown or unexpected char {:?}", ctx.ch);
			}
		}
		ctx.next_char();
	}
	function
}
