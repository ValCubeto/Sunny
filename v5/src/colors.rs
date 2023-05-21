pub fn yellow(text: &str) -> String {
	format!("\x1B[33m{}\x1B[39m", text)
}

pub fn gray(text: &str) -> String {
	format!("\x1B[90m{}\x1B[39m", text)
}

pub fn error(text: &str) -> String {
	format!("\u{1b}[31m\u{1b}[1m{}\u{1b}[22m\u{1b}[39m", text)
}

pub fn warning(text: &str) -> String {
	format!("\u{1b}[31m\u{1b}[1m{}\u{1b}[22m\u{1b}[39m", text)
}