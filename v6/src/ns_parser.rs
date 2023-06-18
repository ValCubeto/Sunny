use std::collections::HashMap;
use crate::context::Context;
use crate::errors::ESYNTAX;
use crate::func_parser::parse_function;
use crate::types::Value;

#[derive(Debug)]
pub struct Namespace {
	data: HashMap<String, Value>
}

impl Namespace {}

pub fn parse_namespace(ctx: &mut Context) -> Namespace {
	let mut namespace = Namespace {
		data: HashMap::new()
	};

	while ctx.idx < ctx.chars.len() {
		println!("{}", ctx.idx);
		ctx.next_char();
		match ctx.ch {
			'\n' => {
				ctx.line += 1;
				ctx.column = 0;
			}
			' ' | '\t' | '\r' => {
				// break
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				let mut word = String::from(ctx.ch);
				ctx.idx += 1;
				// collect word
				while ctx.idx < ctx.chars.len() {
					let ch = ctx.chars[ctx.idx];
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
						let function = parse_function(ctx);
						namespace.data.insert(function.name.clone(), Value::Function(function));
						println!("data = {:?}", namespace.data);
					}
					_ => {
						ctx.throw(ESYNTAX, format!("unexpected identifier {word:?} here"));
					}
				}
			}
			_ => {
				ctx.throw(ESYNTAX, format!("unknown or unexpected char {:?}", ctx.ch));
			}
		}
		ctx.idx += 1;
		ctx.column += 1;
	}
	namespace
}