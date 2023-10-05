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
  ($expr: expr) => {{
    println!(
      "{}: {} = {:?}",
      $crate::bold!($crate::green!("Debug")),
      $crate::bold!(stringify!($expr)),
      $expr
    );
    println!("    at {}:{}:{}", file!(), line!(), column!());
  }};
}

#[macro_export]
macro_rules! warning {
  ($($arg: expr),*) => {
    print!("{}: ", $crate::bold!($crate::yellow!("Warning")));
    println!($($arg),*)
  };
}

#[macro_export]
macro_rules! error {
  ($name: expr; $($arg: expr),* $(; $ctx: expr)?) => {
    print!("{}: ", $crate::bold!($crate::red!($name)));
    println!($($arg),*);
    println!("    at {}:{}:{}", file!(), line!(), column!());
    $( println!("    at {}", $ctx); )?
    std::process::exit(1);
  };
}