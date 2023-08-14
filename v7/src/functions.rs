use crate::{
	// numbers::collect_num,
	context::Context,
	aliases::{ Id, Arguments },
	statments::Statment,
	expressions::parse_expr,
	values::Value,
	syntax_error
};

pub fn parse_function(ctx: &mut Context, name: Id) -> Function {
	let mut function: Vec<Statment> = Vec::new();

	ctx.go();

	if ctx.current == '<' {
		ctx.next_char();
		ctx.go();
		'collect: loop {
			if ctx.current == '>' {
				ctx.next_char();
				break 'collect;
			}
			syntax_error!("function generics to do"; ctx);
			// 'sub: {
			// }
			// ctx.next_char();
			// ctx.go();
		}
	}

	ctx.go();

	if ctx.current != '(' {
		syntax_error!("expected '(', found {:?}", ctx.current; ctx);
	}
	ctx.next_char();
	ctx.go();
	'collect: loop {
		if ctx.current == ')' {
			ctx.next_char();
			break 'collect;
		}
		syntax_error!("function parameters to do"; ctx);
		// 'sub: {
		// }
		// ctx.next_char();
		// ctx.go();
	}
	ctx.go();

	if ctx.current == '-' {
		ctx.next_char();
		if ctx.current != '>' {
			syntax_error!("expected '>' (to complete '->'), found {:?}", ctx.current; ctx);
		}
		ctx.next_char();
		ctx.go();
		syntax_error!("functions' return type to do"; ctx);
	}

	if ctx.current != '{' {
		syntax_error!("expected '{{', found {:?}", ctx.current; ctx);
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
				// match word
				match ctx.current {
					'=' => {
						ctx.next_char();
						let expr = parse_expr(ctx);
						function.push(Statment::Assignment {
							id: word,
							mutable: false,
							expr
						})
					}
					'(' => {
						ctx.next_char();
						loop {
							ctx.go();
							if ctx.current == ')' {
								ctx.next_char();
								break;
							}
							#[allow(unused)]
							let expr = parse_expr(ctx);
							// ctx.next_char();
						}
					}
					_ => syntax_error!("unexpected character {:?}", ctx.current; ctx)
				}
				break 'sub;
			}
			// if ctx.current.is_ascii_digit() {
			// 	let number = collect_num(ctx);
			// 	dbg!(&number);
			// 	break 'sub
			// }

			// match ctx.current {
			// 	// '+' => {}
			// 	_ => 
			// }
			syntax_error!("unexpected character {:?}", ctx.current; ctx)
		}
		ctx.next_char();
		ctx.go();
	}

	Function { name, /* is_async, */ value: FunctionValue::Defined(function) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
	pub name: Id,
	// pub is_async: bool,
	pub value: FunctionValue
}

impl Function {
	pub fn call(&self, args: Arguments) -> Result<Value, Error> {
		match self.value {
			FunctionValue::Builtin(func) => func(args),
			_ => todo!()
		}
	}
}

#[allow(unused)]
#[derive(Debug)]
pub struct Error {
	name: Id,
	description: Id
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum FunctionValue {
	Builtin(fn(Arguments) -> Result<Value, Error>), // Result<Value, &str>
	Defined(Vec<Statment>)
}

