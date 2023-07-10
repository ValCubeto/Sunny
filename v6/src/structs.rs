use crate::{dict::Key, context::Context};

#[derive(Debug)]
pub struct Struct {
	pub name: Key,
	pub properties: Vec<Variable>
}

pub fn parse_struct(ctx: &mut Context) -> Struct {}

pub fn parse_extension(ctx: &mut Context, s: Struct) -> Struct {}