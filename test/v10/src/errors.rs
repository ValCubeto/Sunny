macro_rules! error {
  ($err_name: expr, $( $arg: expr ),*) => {{
    use $crate::terminal::{ bold, red };
    eprintln!("{}: {}.", bold(red($err_name)), format!($( $arg ),*));
    eprintln!("    at {}:{}:{}", file!(), line!(), column!());
    std::process::exit(1);
  }};
}

macro_rules! syn_error {
  ($( $arg: expr ),* $(,)?) => { error!("Syntax error", $( $arg ),*) }
}
macro_rules! ref_error {
  ($( $arg: expr ),* $(,)?) => { error!("Reference error", $( $arg ),*) }
}
