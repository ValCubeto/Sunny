macro_rules! debug {
	($($args:expr),+) => {
		eprint!("[{}] ", crate::colors::yellow("debug"));
		eprintln!($($args),+);
	};
}

macro_rules! Error {
	($name:ident, $($args:expr),+) => {
		eprint!("{}: ", crate::colors::error(stringify!($name)));
		eprintln!($($args),+);
		::std::process::exit(1);
	};
}

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

pub(crate) use debug;
pub(crate) use Error;
pub(crate) use LoadError;
pub(crate) use ArgumentError;