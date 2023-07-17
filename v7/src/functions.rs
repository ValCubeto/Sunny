use crate::{context::Context, id::Id, errors::SyntaxError, statments::Statment, numbers::collect_num, expressions::parse_expr};

pub fn parse_function(ctx: &mut Context, name: Id, is_async: bool) -> Function {
	let mut function = Function::new(name, is_async);

	ctx.go();

	if ctx.current == '<' {
		ctx.next_char();
		ctx.go();
		'collect: loop {
			if ctx.current == '>' {
				ctx.next_char();
				break 'collect;
			}
			'sub: {
				SyntaxError!(ctx, "function generics to do");
			}
			ctx.next_char();
			ctx.go();
		}
	}

	ctx.go();

	if ctx.current != '(' {
		SyntaxError!(ctx, "expected '(', got {:?}", ctx.current);
	}
	ctx.next_char();
	ctx.go();
	'collect: loop {
		if ctx.current == ')' {
			ctx.next_char();
			break 'collect;
		}
		'sub: {
			SyntaxError!(ctx, "function parameters to do");
		}
		ctx.next_char();
		ctx.go();
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
	ctx.go();

	'collect: loop {
		if ctx.current == '}' {
			break 'collect;
		}
		'sub: {
			if ctx.is_valid_id() {
				let word = ctx.collect_word();
				ctx.go();
				match ctx.current {
					'=' => {
						ctx.next_char();
						let expr = parse_expr(ctx);
					}
					_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
				}
				break 'sub;
			}
			// if ctx.current.is_ascii_digit() {
			// 	let number = collect_num(ctx);
			// 	dbg!(&number);
			// 	break 'sub
			// }
			match ctx.current {
				'+' => {}
				_ => SyntaxError!(ctx, "unexpected character {:?}", ctx.current)
			}
		}
		ctx.next_char();
		ctx.go();
	}

	function
}

#[derive(Debug)]
pub struct Function {
	pub name: Id,
	pub is_async: bool,
	pub body: Vec<Statment>
}

impl Function {
	pub fn new(name: Id, is_async: bool) -> Self {
		Function {
			name,
			is_async,
			body: Vec::new()
		}
	}
	// pub fn call(args: Arguments) -> Value {
	// 	eval_code(&self.body, args);
	// }
}