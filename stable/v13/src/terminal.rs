#![allow(unused)]

use chrono::Local;
use std::fmt::Display;
use crate::eval::tokenize::{ COLUMN, LINE, TOK_LEN };
use crate::ctx::{ FILE, CONTENTS };

macro_rules! def_consts {
  ($($name:ident = $value:expr;)*) => {
    $(
      pub const $name: &str = $value;
    )*
  };
}

def_consts! {
  BOLD = "\u{1b}[1m";
  BOLD_END = "\u{1b}[22m";
  ITALIC = "\u{1b}[3m";
  ITALIC_END = "\u{1b}[23m";
  UNDERLINE = "\u{1b}[4m";
  UNDERLINE_END = "\u{1b}[24m";
  RED = "\u{1b}[31m";
  GREEN = "\u{1b}[32m";
  YELLOW = "\u{1b}[33m";
  BLUE = "\u{1b}[34m";
  MAGENTA = "\u{1b}[35m";
  CYAN = "\u{1b}[36m";
  BRIGHT_RED = "\u{1b}[91m";
  BRIGHT_GREEN = "\u{1b}[92m";
  BRIGHT_YELLOW = "\u{1b}[93m";
  BRIGHT_BLUE = "\u{1b}[94m";
  BRIGHT_MAGENTA = "\u{1b}[95m";
  BRIGHT_CYAN = "\u{1b}[96m";
  FOREGROUND_END = "\u{1b}[39m";
  BG_RED = "\u{1b}[41m";
  BG_GREEN = "\u{1b}[42m";
  BG_YELLOW = "\u{1b}[43m";
  BG_BLUE = "\u{1b}[44m";
  BG_MAGENTA = "\u{1b}[45m";
  BG_CYAN = "\u{1b}[46m";
  BG_BRIGHT_RED = "\u{1b}[101m";
  BG_BRIGHT_GREEN = "\u{1b}[102m";
  BG_BRIGHT_YELLOW = "\u{1b}[103m";
  BG_BRIGHT_BLUE = "\u{1b}[104m";
  BG_BRIGHT_MAGENTA = "\u{1b}[105m";
  BG_BRIGHT_CYAN = "\u{1b}[106m";
  BG_END = "\u{1b}[49m";
}

macro_rules! fmt_methods {
  ($(fn $fn_name:ident() => ($start:ident, $end:ident);)*) => {
    $(
      #[inline]
      /// Returns a `String` with the given style to display.
      /// Does nothing if the --no-color flag is passed.
      fn $fn_name(&self) -> String {
        if unsafe { crate::COLORING } {
          format!("{}{}{}", $start, self, $end)
        } else {
          format!("{}", self)
        }
      }
    )*
  }
}

pub trait Stylize: Display {
  fmt_methods! {
    fn bold() => (BOLD, BOLD_END);
    fn underline() => (UNDERLINE, UNDERLINE_END);
    fn red() => (RED, FOREGROUND_END);
    fn green() => (GREEN, FOREGROUND_END);
    fn yellow() => (YELLOW, FOREGROUND_END);
    fn blue() => (BLUE, FOREGROUND_END);
    fn magenta() => (MAGENTA, FOREGROUND_END);
    fn cyan() => (CYAN, FOREGROUND_END);
    fn bright_red() => (BRIGHT_RED, FOREGROUND_END);
    fn bright_green() => (BRIGHT_GREEN, FOREGROUND_END);
    fn bright_yellow() => (BRIGHT_YELLOW, FOREGROUND_END);
    fn bright_blue() => (BRIGHT_BLUE, FOREGROUND_END);
    fn bright_magenta() => (BRIGHT_MAGENTA, FOREGROUND_END);
    fn bright_cyan() => (BRIGHT_CYAN, FOREGROUND_END);
    fn bg_red() => (BG_RED, BG_END);
    fn bg_green() => (BG_GREEN, BG_END);
    fn bg_yellow() => (BG_YELLOW, BG_END);
    fn bg_blue() => (BG_BLUE, BG_END);
    fn bg_magenta() => (BG_MAGENTA, BG_END);
    fn bg_cyan() => (BG_CYAN, BG_END);
    fn bg_bright_red() => (BG_BRIGHT_RED, BG_END);
    fn bg_bright_green() => (BG_BRIGHT_GREEN, BG_END);
    fn bg_bright_yellow() => (BG_BRIGHT_YELLOW, BG_END);
    fn bg_bright_blue() => (BG_BRIGHT_BLUE, BG_END);
    fn bg_bright_magenta() => (BG_BRIGHT_MAGENTA, BG_END);
    fn bg_bright_cyan() => (BG_BRIGHT_CYAN, BG_END);
  }
  fn error(&self) -> String {
    self.red().bold()
  }
  fn warning(&self) -> String {
    self.yellow().bold()
  }
  fn info(&self) -> String {
    self.cyan().bold()
  }
  fn success(&self) -> String {
    self.green().bold()
  }
}

impl<T: Display> Stylize for T {}

pub fn print_debug_msg(msg: &str, file: &str, line: u32, column: u32) {
  if unsafe { !crate::DEBUG } {
    return;
  }
  let path = format!("{}:{}:{}", file, line, column);
  let date = Local::now().format("%d-%m-%y %H:%M:%S");
  println!("[{} at {}, {}]", "Debug".cyan().bold(), path.bold(), date.bold());
  println!("{}", msg);
}

#[macro_export]
macro_rules! debug_msg {
  ($($arg:expr),*) => {{
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    use $crate::terminal::print_debug_msg;
    print_debug_msg(&format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! debug {
  ($arg:expr $(, $arg2:expr),* $(,)?) => {{ 
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    #[allow(unused_mut)]
    let mut msg = format!("{} = {:#?}", stringify!($arg).bold(), $arg);
    $(
      msg.push_str(&format!("; {} = {:#?}", stringify!($arg2).bold(), $arg2));
    ),*
    debug_msg!("{msg}");
  }};
}

#[macro_export]
macro_rules! debug_display {
  ($arg:expr) => {{
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    debug_msg!("{} = {}", stringify!($arg).bold(), $arg);
  }};
}

#[macro_export]
macro_rules! debug_todo {
  ($arg:expr) => {{
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    debug_msg!("{}: {}", "TODO".warning(), $arg);
  }};
}

pub fn quit(ename: &str, msg: &str, file: &str, line: u32, column: u32) -> ! {
  eprintln!("{}: {}", ename.error(), msg);
  unsafe {
    let lines: Vec<&str> = CONTENTS.lines().collect();
    if lines.is_empty() {
      std::process::exit(1);
    }
    let line_text = lines[LINE - 1];
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
  std::process::exit(1);
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
