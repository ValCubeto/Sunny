use std::process::exit;
use crate::ctx::{ CONTENTS, FILE };
use crate::eval::tokenize::{ COLUMN, LINE, TOK_LEN };
use crate::terminal::Stylize;

pub static mut NOTES: String = String::new();

#[macro_export]
macro_rules! note {
  ($arg:expr) => {{
    let mut notes = unsafe { $crate::terminal::NOTES.clone() };
    notes.push_str($arg);
    notes.push('\n');
    // I expect the value it had before to be dropped...
    unsafe { $crate::terminal::NOTES = notes };
  }};
}

pub fn quit(ename: &str, msg: &str, file: &str, line: u32, column: u32) -> ! {
  eprintln!("{}: {}", ename.error(), msg);
  for note in unsafe { NOTES.clone() }.trim_end().lines() {
    eprintln!("  {}: {}", "note".note(), note);
  }
  unsafe {
    let mut lines = CONTENTS.lines();
    let line_text = lines.nth(LINE - 1).unwrap_or_else(|| exit(1));
    if !line_text.trim().is_empty() {
      eprintln!();
      let mut padding = 0;
      for ch in line_text.chars() {
        match ch {
          '\t' => padding += 4,
          ' ' => padding += 1,
          _ => break,
        }
      }
      eprintln!("{}", &line_text[padding..]);
      eprintln!("{}{}", " ".repeat(COLUMN - 1 - padding), "^".repeat(TOK_LEN).red().bold());
    }
    eprintln!("  at {}:{}:{}", FILE, LINE, COLUMN);
    eprintln!("  at {}:{}:{}", file, line, column);
  }
  exit(1);
}

#[macro_export]
macro_rules! internal_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("Internal error", &format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! sys_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("System error", &format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! argument_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("Argument error", &format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! syntax_err {
  ($($arg:expr),*) => {{
    use $crate::terminal::quit;
    quit("Syntax error", &format!($($arg),*), file!(), line!(), column!());
  }};
}
