#[macro_export]
macro_rules! do_error {
	($name:ident, $ctx:ident, $($args:expr),+) => {{
		print!("{}: ", $crate::colors::error(stringify!($name)));
		println!($($args),+);
		println!("    at {}:{}:{}", $ctx.id, $ctx.line, $ctx.column);
		println!("    at {}:{}:{}", file!(), line!(), column!());
		std::process::exit(1);
	}};
	($name:ident, $($args:expr),+) => {{
		print!("{}: ", $crate::colors::error(stringify!($name)));
		println!($($args),+);
		std::process::exit(1);
	}};
}

#[macro_export]
macro_rules! InternalError {
	($($args:expr),+) => {
		$crate::do_error!(InternalError, $($args),+)
	};
}

#[macro_export]
macro_rules! ArgumentError {
	($ctx:ident, $($args:expr),+) => {
		$crate::do_error!(ArgumentError, $ctx, $($args),+)
	};

	($($args:expr),+) => {
		$crate::do_error!(ArgumentError, $($args),+)
	};
}

#[macro_export]
macro_rules! LoadError {
	($ctx:ident, $($args:expr),+) => {
		$crate::do_error!(LoadError, $ctx, $($args),+)
	};

	($($args:expr),+) => {
		$crate::do_error!(LoadError, $($args),+)
	};
}

#[macro_export]
macro_rules! SyntaxError {
	($ctx:ident, $($args:expr),+) => {
		$crate::do_error!(SyntaxError, $ctx, $($args),+)
	};

	($($args:expr),+) => {
		$crate::do_error!(SyntaxError, $($args),+)
	};
}

#[macro_export]
macro_rules! ReferenceError {
	($ctx:ident, $($args:expr),+) => {
		$crate::do_error!(ReferenceError, $ctx, $($args),+)
	};

	($($args:expr),+) => {{
		$crate::do_error!(ReferenceError, $($args),+);
	}};
}

#[macro_export]
macro_rules! TypeError {
	($ctx:ident, $($args:expr),+) => {
		$crate::do_error!(TypeError, $ctx, $($args),+)
	};

	($($args:expr),+) => {
		$crate::do_error!(TypeError, $($args),+)
	};
}

#[allow(unused)]
#[macro_export]
macro_rules! hashmap {
	() => {
		::std::collections::HashMap::new()
	};

	($($key:ident => $value:expr),+ $(,)?) => {
		::std::collections::HashMap::from([ $((::std::rc::Rc::<str>::from(stringify!($key)), $value)),* ])
	};

	($($key:expr => $value:expr),+ $(,)?) => {
		::std::collections::HashMap::from([ $(($key, $value)),* ])
	};
}