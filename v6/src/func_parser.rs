use crate::context::Context;
use crate::errors::SyntaxError;
use crate::params::Params;
use crate::types::{Value, Type};
use crate::word_collector::collect_word;

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
pub struct Function<T = Type> {
	pub name: String,
	params: Params,
	returns: Option<T>,
	pub body: Vec<Statment>
}

pub fn parse_function(ctx: &mut Context) -> Function {
	ctx.ignore_spaces();

	let mut function = Function {
		name: collect_word(ctx),
		params: Params::new(),
		body: Vec::new()
	};

	ctx.ignore_spaces();

	// parse generics

	if ctx.ch != '(' {
		SyntaxError!(ctx, "expected '(', got {:?}", ctx.ch);
	}

	ctx.next_char();
	ctx.ignore_spaces();

	while ctx.idx < ctx.char_count {
		match ctx.ch {
			')' => {
				break;
			}
			'.' => {}
			'a'..='z' | '_' | 'A'..='Z' => {
				let mut param = (collect_word(ctx), Type::Value, Value::None);
				ctx.ignore_spaces();
				match ctx.ch {
					_ => {}
				}

			}
			_ => {
				SyntaxError!(ctx, "expected ')', an identifier, or '...'; got {:?}", ctx.ch);
			}
		}
		ctx.next_char();
	} // parse params
	ctx.next_char();
	ctx.ignore_spaces();
	while ctx.idx < ctx.char_count {
		match ctx.ch {
			'-' => {
				ctx.next_char();
				if ctx.ch != '>' {
					SyntaxError!(ctx, "expected '>' (to complete '->'), got {:?}", ctx.ch);
				}
				ctx.next_char();
				ctx.ignore_spaces();
				SyntaxError!(ctx, "todo: parse types");
			}
			'{' => {
				ctx.next_char();
				ctx.ignore_spaces();
				match ctx.ch {
					'}' => {
						break;
					}
					'a'..='z' | 'A'..='Z' | '_' => {
						let word = collect_word(ctx);
						ctx.ignore_spaces();
						match word.as_str() {
							"if" => {
								SyntaxError!(ctx, "todo: if");
							}
							_ => {
								match ctx.ch {
									'=' => {
										SyntaxError!(ctx, "todo: assignment");
									}
									_ => {
										SyntaxError!(ctx, "unexpected character {:?}", ctx.ch);
									}
								}
							} // identifier
						} // match word
					} // collect word
					'+' => {
						SyntaxError!(ctx, "todo: add-assign");
					}
					'-' => {
						SyntaxError!(ctx, "todo: sub-assign");
					}
					_ => {
						SyntaxError!(ctx, "unexpected character {:?}", ctx.ch);
					}
				}
			} // '{'
			_ => {
				SyntaxError!(ctx, "expected '->' or '{{', got {:?}", ctx.ch);
			}
		}
	}
	println!("end of function {:?}", function.name);
	function
}
