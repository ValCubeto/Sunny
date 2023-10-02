/// Create a HashMap in a cooler way using the arrow syntax
#[macro_export]
macro_rules! hashmap {
  () => {
    ::std::collections::HashMap::new()
  };
  ($($key: expr => $value: expr),* $(,)?) => {
    ::std::collections::HashMap::from([
      $(
        ($key, $value)
      ),*
    ])
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
  ($($args: expr),*) => {{
    print!("{}: ", $crate::bold!($crate::green!("Debug")));
    println!($($args),*);
  }};
}

#[macro_export]
macro_rules! error {
  ($name: expr, ) => {
    todo!()
  };
}