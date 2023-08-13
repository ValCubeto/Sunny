use std::collections::HashMap;
use crate::{
	context::Context,
	aliases::Id,
	values::Value,
	// functions::{ FunctionValue, parse_function },
	syntax_error, reference_error
};

pub fn parse_namespace(ctx: &mut Context, name: Id) -> Namespace {
	let mut namespace = Namespace::new(name);

	ctx.go();
	if ctx.current != '{' {
		syntax_error!("expected '{{', found {:?}", ctx.current; ctx);
	}
	
	ctx.next_char();
	ctx.go();

	'collect: loop {
		if ctx.current == '}' {
			break 'collect;
		}
		let word: Id = ctx.expect_word();
		match Id::as_ref(&word) {
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
				namespace.set(name, Value::Function(Box::new(FunctionValue::Defined(value))));
			}
			"async" | "struct" | "extend" | "const" | "import" => syntax_error!("not implemented"; ctx),
			_ => syntax_error!("unexpected identifier {word:?} here"; ctx)
		}
		ctx.next_char();
		ctx.go();
	}

	namespace
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace {
	pub name: Id,
	pub data: HashMap<Id, Value>
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
			reference_error!("{:?} already defined as a {}", id, self.data[&id].typename());
		}
		self.data.insert(id, value);
	}
}