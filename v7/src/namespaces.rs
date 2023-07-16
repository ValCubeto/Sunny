use std::collections::HashMap;
use crate::{context::Context,
	id::Id,
	values::Value};

pub fn parse_namespace(ctx: &mut Context, id: Id) -> Namespace {
	let mut namespace = Namespace::new(id);

	ctx.go(false);

	namespace
}

pub struct Namespace {
	id: Id,
	data: HashMap<Id, Value>
}

impl Namespace {
	pub fn new(id: Id) -> Self {
		Namespace {
			id,
			data: HashMap::new()
		}
	}
}