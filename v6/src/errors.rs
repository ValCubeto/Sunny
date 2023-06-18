pub const EINTERNAL: &str = "InternalError";
pub const EARGS: &str = "ArgumentError";
pub const ELOAD: &str = "LoadError";
pub const ESYNTAX: &str = "SyntaxError";

macro_rules! do_error {
	($($args:expr),*) => {
		println!()
	};
}

pub(crate) use SyntaxError;