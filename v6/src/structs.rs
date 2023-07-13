use crate::{
	dict::Id,
	context::Context,
	types::{Value, Type}, errors::SyntaxError
};

#[derive(Debug)]
pub struct Struct {
	pub name: Id,
	pub properties: Vec<(bool, Type, Value)>
}

pub fn parse_struct(ctx: &mut Context) -> Struct {
	let mut s = Struct {
		name: ctx.collect_word(),
		properties: Vec::new()
	};
	s
}

pub fn parse_extension<'a>(ctx: &mut Context, s: &'a mut Struct) -> &'a mut Struct {
	ctx.ignore_spaces();
	// parse generics
	if ctx.ch != '{' {
		SyntaxError!(ctx, "expected '{{', got {:?}", ctx.ch);
	}
	ctx.ignore_spaces();
	todo!("w");
	s
}

#[derive(Debug)]
pub struct Instance {
	pub parent: Id,
	pub values: Vec<(bool, Type, Value)>
}