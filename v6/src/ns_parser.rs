use std::collections::HashMap;
use crate::context::Context;
use crate::dict::Key;
use crate::errors::{SyntaxError, ReferenceError};
use crate::func_parser::parse_function;
use crate::types::Value;

#[derive(Debug)]
pub struct Namespace {
	name: Key,
	data: HashMap<Key, Value>
}

impl Namespace {
	pub fn set(&mut self, ctx: &mut Context, k: Key, v: Value) {
		if self.data.contains_key(&k) {
			ReferenceError!(ctx, "the {} {k} is already defined", match v {
				Value::Function(_) => "function",
				Value::Namespace(_) => "namespace",
				_ => "variable"
			});
		}
		self.data.insert(k, v);
	}
}

pub fn parse_namespace(ctx: &mut Context) -> Namespace {
	ctx.ignore_spaces();

	let mut namespace = Namespace {
		name: ctx.collect_word(),
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
				let word = ctx.collect_word();
				dbg!(&word);
				match word.as_str() {
					"fun" => {
						let function = parse_function(ctx);
						namespace.set(ctx, function.name.clone(), Value::Function(function));
					}
					"struct" | "extend" => {
						SyntaxError!(ctx, "structs to do");
					}
					"const" => {
						SyntaxError!(ctx, "constants to do");
					}
					"import" => {
						SyntaxError!(ctx, "imports to do");
					}
					"namespace" => {
						let nested = parse_namespace(ctx);
						namespace.set(ctx, nested.name.clone(), Value::Namespace(nested));
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
				SyntaxError!(ctx, "unexpected char {:?}", ctx.ch);
			}
		}
		ctx.next_char();
	}

	namespace
}