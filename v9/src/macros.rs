/// Create a HashMap in a cooler way using the arrow syntax
#[macro_export]
macro_rules! hashmap {
  () => {
    ::std::collections::HashMap::new()
  };
  ($($key: expr => $value: expr),* $(,)?) => {
    ::std::collections::HashMap::from([
      $( ($key, $value) ),*
    ])
  };
}

/// Create a HashMap in a cooler way using the arrow syntax
#[macro_export]
macro_rules! dict {
  () => {
    $crate::values::Value::Dict(
      $crate::aliases::DictPtr::new(
        ::std::collections::HashMap::new()
      )
    )
  };
  ($($key: expr => $value: expr),* $(,)?) => {
    $crate::values::Value::Dict(
      $crate::aliases::DictPtr::new(
        ::std::collections::HashMap::from([
          $( ($key, $value) ),*
        ])
      )
    )
  };
}

#[macro_export]
macro_rules! bold {
  ($text: expr) => {
    format!("{}{}{}", $crate::colors::BOLD, $text, $crate::colors::BOLD_END)
  };
}

#[macro_export]
macro_rules! green {
  ($text: expr) => {
    format!("{}{}{}", $crate::colors::GREEN, $text, $crate::colors::COLOR_END)
  };
}

#[macro_export]
macro_rules! yellow {
  ($text: expr) => {
    format!("{}{}{}", $crate::colors::YELLOW, $text, $crate::colors::COLOR_END)
  };
}

#[macro_export]
macro_rules! red {
  ($text: expr) => {
    format!("{}{}{}", $crate::colors::RED, $text, $crate::colors::COLOR_END)
  };
}

#[macro_export]
macro_rules! debug {
  ($($value: expr),*) => {{
    $(
      println!(
        "{}: {} = {:#?}",
        $crate::bold!($crate::green!("Debug")),
        $crate::bold!(stringify!($value)),
        $value
      );
    )*
    println!("    at {}:{}:{}", file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! warning {
  ($($arg: expr),*) => {{
    print!("{}: ", $crate::bold!($crate::yellow!("Warning")));
    println!($($arg),*)
  }};
}

#[macro_export]
macro_rules! error {
  ($name: expr; $($arg: expr),* $(; $ctx: expr)?) => {{
    eprint!("{}: ", $crate::bold!($crate::red!($name)));
    eprintln!($($arg),*);
    eprintln!("    at {}:{}:{}", file!(), line!(), column!());
    $( eprintln!("    at {}:{}:{}", $ctx.src, $ctx.line, $ctx.column); )?
    ::std::process::exit(1);
  }};
}

#[macro_export]
macro_rules! internal_error {
  ($( $arg: expr ),* $( ; $ctx: expr )?) => {
    $crate::error!("InternalError"; $( $arg )* $( ; $ctx )?)
  };
}

#[macro_export]
macro_rules! argument_error {
  ($( $arg: expr ),* $( ; $ctx: expr )?) => {
    $crate::error!("ArgumentError"; $( $arg )* $( ; $ctx )?)
  };
}

#[macro_export]
macro_rules! load_error {
  ($( $arg: expr ),* $( ; $ctx: expr )?) => {
    $crate::error!("LoadError"; $( $arg )* $( ; $ctx )?)
  };
}

#[macro_export]
macro_rules! syntax_error {
  ($( $arg: expr ),* $( ; $ctx: expr )?) => {
    $crate::error!("SyntaxError"; $( $arg )* $( ; $ctx )?)
  };
}

#[macro_export]
macro_rules! reference_error {
  ($( $arg: expr ),* $( ; $ctx: expr )?) => {
    $crate::error!("ReferenceError"; $( $arg )* $( ; $ctx )?)
  };
}

#[macro_export]
macro_rules! type_error {
  ($( $arg: expr ),* $( ; $ctx: expr )?) => {
    $crate::error!("TypeError"; $( $arg )* $( ; $ctx )?)
  };
}
