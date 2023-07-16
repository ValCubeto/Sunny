use std::collections::HashMap;
use crate::{context::Context,
	id::Id,
	values::Value, errors::{SyntaxError, ReferenceError}, functions::parse_function};

pub fn parse_namespace(ctx: &mut Context, name: Id) -> Namespace {
	let mut namespace = Namespace::new(name);

	ctx.go();
	if ctx.current != '{' {
		SyntaxError!(ctx, "expected '{{', got {:?}", ctx.current);
	}

	ctx.go();

	'collect: loop {
		if ctx.current == '}' {
			break 'collect;
		}
		'sub: {
			if ctx.current.is_alphabetic() {
				let word: Id = ctx.collect_word();
				match word.to_str() {
					"namespace" => {
						let name = ctx.collect_word();
						let value = parse_namespace(ctx, name.clone());
						namespace.set(name, Value::Namespace(Box::new(value)));
					}
					"function" => {
						let name = ctx.collect_word();
						let value = parse_function(ctx, name.clone(), false);
						namespace.set(name, Value::Function(Box::new(value)));
					}
					"struct" | "extend" | "const" => SyntaxError!(ctx, "to do"),
					_ => SyntaxError!(ctx, "unexpected identifier {word:?} here")
				}
				break 'sub;
			}
			SyntaxError!(ctx, "expected a keyword, got {:?}", ctx.current);
		}
		ctx.next_char();
	}

	namespace
}

#[allow(unused)]
pub struct Namespace {
	name: Id,
	data: HashMap<Id, Value>
}

impl Namespace {
	pub fn new(name: Id) -> Self {
		Namespace {
			name,
			data: HashMap::new()
		}
	}
	pub fn get(&self, id: &Id) -> Option<&Value> {
		self.data.get(id)
	}
	pub fn set(&mut self, id: Id, value: Value) {
		if self.data.contains_key(&id) {
			ReferenceError!("{:?} already defined as a {}", id, self.data[&id].typename());
		}
		self.data.insert(id, value);
	}
}