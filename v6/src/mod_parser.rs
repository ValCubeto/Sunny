use std::collections::HashMap;
use crate::context::Context;
use crate::errors::SyntaxError;
use crate::func_parser::parse_function;
use crate::types::Value;

#[derive(Debug)]
pub struct Module {
	data: HashMap<String, Value>
}

impl Module {}

pub fn parse_module(code: String, id: String) -> Module {
	let mut module = Module {
		data: HashMap::new()
	};

	let ctx = &mut Context::new(id.clone(), code.chars().collect());

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
						module.data.insert(function.name.clone(), Value::Function(function));
						println!("data = {:?}", module.data);
					}
					_ => {
						SyntaxError!("unexpected identifier {word:?} here")
					}
				}
			}
			_ => {
				SyntaxError!("unknown or unexpected char {:?}\n    at {id}:{}:{}", ctx.ch, ctx.line, ctx.column);
			}
		}
		ctx.idx += 1;
		ctx.column += 1;
	}

	module
}