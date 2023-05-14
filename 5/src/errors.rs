macro_rules! debug {
	($($args:expr),+) => {
		eprint!("[{}] ", crate::colors::yellow("debug"));
		eprintln!($($args),+);
	};
}

// imagina una macro que crea macros y las exporta...

/// flaco poné el punto y coma o se pudre todo
macro_rules! Error {
	($name:ident, $($args:expr),+) => {
		eprint!("{}: ", crate::colors::error(stringify!($name)));
		eprintln!($($args),+);
		::std::process::exit(1);
	};
}

/// flaco poné el punto y coma o se pudre todo
macro_rules! LoadError {
	($($args:expr),+) => {
		crate::errors::Error!(LoadError, $($args),+);
	};
}

macro_rules! ArgumentError {
	($($args:expr),+) => {
		crate::errors::Error!(ArgumentError, $($args),+);
	};
}

#[allow(unused)]
macro_rules! InternalError {
	($($args:expr),+) => {
		crate::errors::Error!(InternalError, $($args),+);
	};
}

macro_rules! SyntaxError {
	($($args:expr),+) => {
		crate::errors::Error!(InternalError, $($args),+);
	};
}

pub(crate) use debug;
pub(crate) use Error;
pub(crate) use LoadError;
pub(crate) use ArgumentError;
#[allow(unused)]
pub(crate) use InternalError;
pub(crate) use SyntaxError;