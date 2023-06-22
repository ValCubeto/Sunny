use std::collections::HashMap;
use crate::context::Context;
use crate::errors::{SyntaxError, ReferenceError};
use crate::func_parser::parse_function;
use crate::types::Value;

#[allow(unused)]
#[derive(Debug)]
pub struct Namespace {
	name: String,
	data: HashMap<String, Value>
}

impl Namespace {}

pub fn parse_namespace(ctx: &mut Context) -> Namespace {
	let mut namespace = Namespace {
		name: String::new(),
		data: HashMap::new()
	};

	// collect name

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
						if namespace.data.contains_key(&function.name) {
							ReferenceError!(ctx, "identifier {:?} already used", function.name);
						}
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
				// handled by next_char
			}
		}
		ctx.next_char();
	}
	namespace
}