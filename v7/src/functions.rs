use crate::{context::Context, id::Id, errors::SyntaxError};

pub fn parse_function(ctx: &mut Context, name: Id, is_async: bool) -> Function {
	let mut function = Function::new(name, is_async);

	ctx.go();

	if ctx.current == '<' {
		'collect: loop {
			if ctx.current == '>' {
				ctx.next_char();
				break 'collect;
			}
			'sub: {
				SyntaxError!(ctx, "function generics to do");
			}
			ctx.next_char();
		}
	}

	ctx.go();

	if ctx.current != '(' {
		SyntaxError!(ctx, "expected '(', got {:?}", ctx.current);
	}
	ctx.next_char();
	'collect: loop {
		if ctx.current == ')' {
			ctx.next_char();
			break 'collect;
		}
		'sub: {
			SyntaxError!(ctx, "function parameters to do");
		}
		ctx.next_char();
	}
	ctx.go();

	if ctx.current == '-' {
		ctx.next_char();
		if ctx.current != '>' {
			SyntaxError!(ctx, "expected '>' (to complete '->'), got {:?}", ctx.current);
		}
		ctx.next_char();
		ctx.go();
		SyntaxError!(ctx, "functions' return type to do");
	}

	if ctx.current != '{' {
		SyntaxError!(ctx, "expected '{{', got {:?}", ctx.current);
	}
	ctx.next_char();

	'collect: loop {
		if ctx.current == '}' {
			
		}
	}

	function
}

pub struct Function {
	pub name: Id,
	pub is_async: bool
}

impl Function {
	pub fn new(name: Id, is_async: bool) -> Self {
		Function {
			name,
			is_async
		}
	}
}