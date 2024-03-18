
/// Generic macro to print errors. Used by the specific-error macros
macro_rules! err {
  ($err_name: expr, $($err_msg: expr),* $(; $parser: expr)?) => {{
    eprintln!("{}: {}", $crate::colors::bold($crate::colors::red($err_name)), format!($($err_msg),*));
    // NOTE: sometimes is not possible to access the parser
    $(
      eprintln!("    at {}:{}:{}", $parser.file_name, $parser.line, $parser.column);
    )?
    ::std::process::exit(1);
  }};
}

/// Macro for printing syntax errors
macro_rules! syn_err {
  ($($arg: expr),* $(; $parser: expr)?) => {
    err!("Syntax error", $($arg),* $(; $parser)?)
  };
}

/// Macro for printing range errors
macro_rules! ran_err {
  ($($arg: expr),* $(; $parser: expr)?) => {
    err!("Range error", $($arg),* $(; $parser)?)
  };
}
