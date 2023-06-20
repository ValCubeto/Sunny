// const RESET: &str = "\u{1b}[0m";

const BOLD: &str = "\u{1b}[1m";
const BOLD_END: &str = "\u{1b}[22m";

const RED: &str = "\u{1b}[31m";
#[allow(unused)]
const YELLOW: &str = "\u{1b}[33m";
const COLOR_END: &str = "\u{1b}[39m";

pub fn error(text: &str) -> String {
	format!("{}{}{}{}{}", BOLD, RED, text, COLOR_END, BOLD_END)
}

#[allow(unused)]
pub fn warning(text: &str) -> String {
	format!("{}{}{}{}{}", BOLD, YELLOW, text, COLOR_END, BOLD_END)
}