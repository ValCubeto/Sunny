use crate::{
	dict::Id,
	context::Context
};

#[derive(Debug)]
pub struct Struct {
	pub name: Id,
	pub properties: Vec<Variable>
}

pub fn parse_struct(ctx: &mut Context) -> Struct {}

pub fn parse_extension(ctx: &mut Context, s: &mut Struct) -> Struct {}

#[derive(Debug)]
pub struct Instance {
	pub parent: Id,
	pub values: Vec<Variable>
}