use std::collections::HashMap;
use crate::context::Context;
use crate::dict::Key;
use crate::errors::{SyntaxError, ReferenceError};
use crate::func_parser::parse_function;
use crate::types::Value;
use crate::word_collector::collect_word;

#[allow(unused)]
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
				match &*word.0 {
					"fun" => {
						let function = parse_function(ctx);
						namespace.set(ctx, function.name, Value::Function(function));
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
				SyntaxError!(ctx, "unexpected {:?}", ctx.ch);
			}
		}
		ctx.next_char();
	}
	namespace
}