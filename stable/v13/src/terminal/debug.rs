use std::sync::atomic::Ordering::Relaxed;
use chrono::Local;
use crate::terminal::Stylize;

pub fn print_debug_msg(msg: &str, file: &str, line: u32, column: u32) {
  if !crate::DEBUG.load(Relaxed) {
    return;
  }
  let path = format!("{}:{}:{}", file, line, column);
  let date = Local::now().format("%d-%m-%y %H:%M:%S").to_string();
  println!("[{} at {}, {}]", "Debug".info(), path.bold(), date.bold());
  println!("{}", msg);
}

#[macro_export]
macro_rules! debug_msg {
  ($($arg:expr),*) => {{
    use $crate::terminal::print_debug_msg;
    print_debug_msg(&format!($($arg),*), file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! debug {
  ($($arg:expr),+ $(,)?) => {{ 
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    #[allow(unused_mut)]
    let mut msg = vec![
      $( format!("{} = {:?}", stringify!($arg).bold(), $arg)),+
    ];
    debug_msg!("{}", msg.join("; "));
  }};
}


#[macro_export]
macro_rules! debug_pretty {
  ($($arg:expr),+ $(,)?) => {{ 
    #[allow(unused_imports)]
    use $crate::terminal::Stylize;
    #[allow(unused_mut)]
    let mut msg = vec![
      $( format!("{} = {:#?}", stringify!($arg).bold(), $arg)),+
    ];
    debug_msg!("{}", msg.join("; "));
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
