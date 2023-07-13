use std::collections::HashMap;
use crate::{
	context::Context,
	dict::Id,
	errors::{SyntaxError, ReferenceError, InternalError, TypeError},
	func_parser::parse_function,
	structs::{parse_struct, parse_extension},
	types::Value,
};

#[derive(Debug)]
pub struct Namespace {
	name: Id,
	values: HashMap<Id, Value>
}

impl Namespace {
	pub fn set(&mut self, ctx: &mut Context, k: Id, v: Value) {
		if self.values.contains_key(&k) {
			ReferenceError!(ctx, "{k} is already defined");
		}
		self.values.insert(k, v);
	}
}

pub fn parse_namespace(ctx: &mut Context) -> Namespace {
	ctx.ignore_spaces();

	let mut namespace = Namespace {
		name: ctx.collect_word(),
		values: HashMap::new()
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
					"struct" => {
						let value = parse_struct(ctx);
						namespace.set(ctx, value.name.clone(), Value::Struct(value));
					}
					"extend" => {
						ctx.ignore_spaces();
						let id = ctx.collect_word();
						if !namespace.values.contains_key(&id) {
							SyntaxError!(ctx, "extend before initialization todo");
						}
						let value = match namespace.values.get(&id) {
							None => {
								InternalError!("bro");
							}
							Some(value) => {
								match value {
									Value::Struct(value) => *value,
									_ => {
										TypeError!(ctx, "trying to extend a value that is not a struct");
									}
								}
							}
						};
						parse_extension(ctx, value);
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