use crate::context::Context;
use crate::errors::ESYNTAX;
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
			' ' | '\t' | '\r' => {
				// ignore
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				function.name.push(ctx.ch);
				ctx.next_char();
				while ctx.idx < ctx.chars.len() {
					match ctx.ch {
						'a'..='z' | 'A'..='Z' | '_' => {
							function.name.push(ctx.ch);
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
				ctx.throw(ESYNTAX, format!("unexpected char {:?}", ctx.ch));
			}
		}
	}
	function
}
