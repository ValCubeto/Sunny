use std::collections::HashMap;
use crate::{context::Context,
	id::Id,
	values::Value,
	errors::{SyntaxError, ReferenceError},
	functions::parse_function};

pub fn parse_namespace(ctx: &mut Context, name: Id) -> Namespace {
	let mut namespace = Namespace::new(name);

	ctx.go();
	if ctx.current != '{' {
		SyntaxError!(ctx, "expected '{{', got {:?}", ctx.current);
	}
	
	ctx.next_char();
	ctx.go();

	'collect: loop {
		if ctx.current == '}' {
			break 'collect;
		}
		let word: Id = ctx.expect_word();
		match word.to_str() {
			"namespace" => {
				ctx.go();
				let name = ctx.expect_word();
				let value = parse_namespace(ctx, name.clone());
				namespace.set(name, Value::Namespace(Box::new(value)));
			}
			"fun" => {
				ctx.go();
				let name = ctx.expect_word();
				let value = parse_function(ctx, name.clone(), false);
				namespace.set(name, Value::Function(Box::new(value)));
			}
			"async" => {
				ctx.go();
				let kw = ctx.expect_word();
				if kw.to_str() != "fun" {
					SyntaxError!(ctx, "expected keyword 'function'")
				}
				ctx.go();
				let name = ctx.expect_word();
				let value = parse_function(ctx, name.clone(), true);
				namespace.set(name, Value::Function(Box::new(value)));
			}
			"struct" | "extend" | "const" | "import" => SyntaxError!(ctx, "not implemented"),
			_ => SyntaxError!(ctx, "unexpected identifier {word:?} here")
		}
		ctx.next_char();
		ctx.go();
	}

	namespace
}

#[derive(Debug)]
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