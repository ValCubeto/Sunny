use std::collections::HashMap;
use crate::context::Context;
use crate::errors::{SyntaxError, ReferenceError};
use crate::func_parser::parse_function;
use crate::types::Value;
use crate::word_collector::collect_word;

#[allow(unused)]
#[derive(Debug)]
pub struct Namespace {
	name: String,
	data: HashMap<String, Value>
}

impl Namespace {}

pub fn parse_namespace(ctx: &mut Context) -> Namespace {
	let mut namespace = Namespace {
		name: collect_word(ctx),
		data: HashMap::new()
	};
	dbg!(&namespace.name);

	ctx.ignore_spaces();

	if ctx.ch != '{' {
		SyntaxError!(ctx, "expected '{{', got {:?}", ctx.ch);
	}
	ctx.next_char();

	while ctx.idx < ctx.char_count {
		ctx.ignore_spaces();
		println!("{ctx:#}");
		match ctx.ch {
			'a'..='z' | 'A'..='Z' | '_' => {
				let word = collect_word(ctx);
				dbg!(&word);
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
						SyntaxError!(ctx, "struct: todo");
					}
					"const" => {
						SyntaxError!(ctx, "const: todo");
					}
					"namespace" => {
						ctx.ignore_spaces();
						let nested = parse_namespace(ctx);
						// check if already declared
						namespace.data.insert(nested.name.clone(), Value::Namespace(nested));
					}
					_ => {
						SyntaxError!(ctx, "unexpected identifier {word:?} here");
					}
				}
			}
			'}' => {
				println!("end of namespace {:?}", namespace.name);
				break;
			}
			_ => {
				SyntaxError!(ctx, "unexpected {:?}", ctx.ch);
			}
		}
		ctx.next_char();
	}
	namespace
}