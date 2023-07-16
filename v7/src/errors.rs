macro_rules! do_error {
	($name:ident, $ctx:ident, $($args:expr),+) => {{
		print!("{}: ", crate::colors::error(stringify!($name)));
		println!($($args),+);
		println!("    at {:?}:{}:{}", $ctx.id, $ctx.line, $ctx.column);
		println!("    at {}:{}:{}", file!(), line!(), column!());
		std::process::exit(1);
	}};
	($name:ident, $($args:expr),+) => {{
		print!("{}: ", crate::colors::error(stringify!($name)));
		println!($($args),+);
		std::process::exit(1);
	}};
}

macro_rules! InternalError {
	($($args:expr),+) => {{
		crate::errors::do_error!(InternalError, $($args),+);
	}};
}

macro_rules! ArgumentError {
	($ctx:ident, $($args:expr),+) => {{
		crate::errors::do_error!(ArgumentError, $ctx, $($args),+);
	}};
	($($args:expr),+) => {{
		crate::errors::do_error!(ArgumentError, $($args),+);
	}};
}

macro_rules! LoadError {
	($ctx:ident, $($args:expr),+) => {{
		crate::errors::do_error!(LoadError, $ctx, $($args),+);
	}};
	($($args:expr),+) => {{
		crate::errors::do_error!(LoadError, $($args),+);
	}};
}

#[allow(unused)]
macro_rules! SyntaxError {
	($ctx:ident, $($args:expr),+) => {{
		crate::errors::do_error!(SyntaxError, $ctx, $($args),+);
	}};
	($($args:expr),+) => {{
		crate::errors::do_error!(SyntaxError, $($args),+);
	}};
}

#[allow(unused)]
macro_rules! ReferenceError {
	($ctx:ident, $($args:expr),+) => {{
		crate::errors::do_error!(ReferenceError, $ctx, $($args),+);
	}};
	($($args:expr),+) => {{
		crate::errors::do_error!(ReferenceError, $($args),+);
	}};
}

#[allow(unused)]
macro_rules! TypeError {
	($ctx:ident, $($args:expr),+) => {{
		crate::errors::do_error!(TypeError, $ctx, $($args),+);
	}};
	($($args:expr),+) => {{
		crate::errors::do_error!(TypeError, $($args),+);
	}};
}

pub(crate) use {
	do_error,
	InternalError,
	ArgumentError,
	LoadError,
	SyntaxError,
	ReferenceError,
	TypeError
};