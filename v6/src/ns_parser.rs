use std::collections::HashMap;
use crate::context::Context;
use crate::errors::SyntaxError;
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
		println!("{ctx:#}");
		match ctx.ch {
			'\n' | ' ' | '\t' | '\r' => {
				// break
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				let mut word = String::from(ctx.ch);
				ctx.next_char();
				while ctx.idx < ctx.chars.len() {
					println!("{ctx:#}");
					match ctx.ch {
						'a'..='z' | 'A'..='Z' | '_' => {
							word.push(ctx.ch);
						}
						_ => {
							ctx.next_char();
							break;
						}
					}
					ctx.next_char();
				}
				match word.as_str() {
					"fun" => {
						let function = parse_function(ctx);
						namespace.data.insert(function.name.clone(), Value::Function(function));
						println!("data = {:?}", namespace.data);
					}
					"struct" | "extend" => {
						SyntaxError!(ctx, "structs not implemented yet");
					}
					"const" => {
						SyntaxError!(ctx, "const");
					}
					_ => {
						SyntaxError!(ctx, "unexpected identifier {word:?} here");
					}
				}
			}
			_ => {
				SyntaxError!(ctx, "unknown or unexpected char {:?}", ctx.ch);
			}
		}
		ctx.next_char();
	}
	namespace
}