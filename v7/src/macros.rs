#[macro_export]
macro_rules! error {
  ($error_name:expr; $($arg:expr),*) => {{
    eprint!("{}: ", $crate::display_bold!($crate::display_red!($error_name)));
    eprintln!($($arg),*);
    eprintln!("    at {}:{}:{}", file!(), line!(), column!());
    std::process::exit(1);
  }};
  ($error_name:expr; $($arg:expr),*; $ctx:expr) => {{
    eprint!("{}: ", $crate::display_bold!($crate::display_red!($error_name)));
    eprintln!($($arg),*);
    eprintln!("    at {}:{}:{}", $ctx.id, $ctx.line, $ctx.column);
    eprintln!("    at {}:{}:{}", file!(), line!(), column!());
    std::process::exit(1);
  }};
}

#[macro_export]
macro_rules! internal_error {
  ($($arg:expr),*) => { $crate::error!("InternalError"; $($arg),*) }
}

#[macro_export]
macro_rules! argument_error {
  ($($arg:expr),*) => { $crate::error!("ArgumentError"; $($arg),*) }
}

#[macro_export]
macro_rules! load_error {
  ($($arg:expr),*) => { $crate::error!("LoadError"; $($arg),*) }
}

#[macro_export]
macro_rules! syntax_error {
  ($($arg:expr),*; $ctx:expr) => { $crate::error!("SyntaxError"; $($arg),*; $ctx) };
  ($($args:expr),+) => { $crate::error!("SyntaxError"; $($args),+) };
}

#[macro_export]
macro_rules! reference_error {
  ($($args:expr),+; $ctx:expr) => { $crate::error!("ReferenceError"; $($args),+; $ctx) };
  ($($args:expr),+) => { $crate::error!("ReferenceError"; $($args),+) };
}

#[macro_export]
macro_rules! type_error {
  ($($args:expr),+; $ctx:expr) => { $crate::error!("TypeError"; $($args),+; $ctx) };
  ($($args:expr),+) => { $crate::error!("TypeError"; $($args),+) };
}

#[macro_export]
macro_rules! display_red {
  ($text:expr) => { format!("{}{}{}", $crate::colors::RED, $text, $crate::colors::COLOR_END) }
}

#[macro_export]
macro_rules! display_bold {
  ($text:expr) => { format!("{}{}{}", $crate::colors::BOLD, $text, $crate::colors::BOLD_END) }
}

#[macro_export]
/// Creates a HashMap. It's a shorthand for the following code
/// ```rs
/// HashMap::from([ (key, value) ])
/// ```
macro_rules! hashmap {
  () => {
    ::std::collections::HashMap::new()
  };

  ($($key:expr => $value:expr),+ $(,)?) => {
    ::std::collections::HashMap::from([ $(($key, $value)),* ])
  };
}

#[macro_export]
/// ```rust
///
/// ($name:expr, $closure:expr)
///
/// Value::Function(
///     Rc::new(
///         Function {
///             name: Id::from($name),
///             value: FunctionValue::Builtin($closure)
///         }
///     )
/// )
/// ```
macro_rules! builtin_function {
  ($name:expr, $closure:expr) => {
    Value::Function(std::rc::Rc::new(Function {
      name: Id::from($name),
      value: FunctionValue::Builtin($closure)
    }))
  };
}
