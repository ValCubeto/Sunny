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
		println!("{}", ctx.idx);
		ctx.next_char();
		match ctx.ch {
			' ' | '\t' | '\r' => {
				// break
			}
			'a'..='z' | 'A'..='Z' | '_' => {
				let mut word = String::from(ctx.ch);
				// ctx.idx += 1;
				// collect word
				while ctx.idx < ctx.chars.len() {
					println!("idx = {}", ctx.idx);
					ctx.next_char();
					match ctx.ch {
						'a'..='z' | 'A'..='Z' | '_' => {
							ctx.column += 1;
							word.push(ctx.ch);
						}
						_ => {
							ctx.idx -= 1;
							break;
						}
					}
				}
				match word.as_str() {
					"fun" => {
						ctx.idx += 1;
						let function = parse_function(ctx);
						namespace.data.insert(function.name.clone(), Value::Function(function));
						println!("data = {:?}", namespace.data);
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
		ctx.idx += 1;
		ctx.column += 1;
	}
	namespace
}