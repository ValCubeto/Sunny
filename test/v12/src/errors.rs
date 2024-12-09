
/// Generic macro to print errors. Used by the specific-error macros
macro_rules! err {
  ($err_name: expr, $($err_msg: expr),* $(; $parser: expr)?) => {{
    eprintln!("{}: {}", $crate::colors::bold($crate::colors::red($err_name)), format!($($err_msg),*));
    // NOTE: sometimes is not possible to access the parser
    $(
      eprintln!("    at {}:{}:{}", $parser.path().display(), $parser.line(), $parser.column());
    )?
    eprintln!("    at {}:{}:{}", file!(), line!(), column!());
    ::std::process::exit(1);
  }};
}

/// Macro for printing syntax errors
macro_rules! internal_err {
  ($($arg: expr),* $(; $parser: expr)?) => {
    err!("Internal error", $($arg),* $(; $parser)?)
  };
}

/// Macro for printing syntax errors
macro_rules! syntax_err {
  ($($arg: expr),* $(; $parser: expr)?) => {
    err!("Syntax error", $($arg),* $(; $parser)?)
  };
}

/// Macro for printing range errors
/// Example of input: `let a: u8 = 256`
/// Expected output: `Range error: unsigned 8-bit integer literals must be between 0 and 256`
macro_rules! range_err {
  ($($arg: expr),* $(; $parser: expr)?) => {
    err!("Range error", $($arg),* $(; $parser)?)
  };
}
