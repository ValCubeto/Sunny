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

#[allow(unused)]
#[macro_export]
/// Creates a HashMap
/// 
/// ```rs
/// hashmap! {
///     key => "value"
/// }
/// ```
/// is the same as:
/// ```rs
/// HashMap::from([ Rc::<str>::from("key"), "value" ])
/// ```
/// 
/// [```Rc<str>```](std::rc::Rc) is used because you will copy the key reference instead of the value and the keys are inmutable so we use str instead of String
macro_rules! hashmap {
  () => {
    ::std::collections::HashMap::new()
  };

  ($($key:ident => $value:expr),+ $(,)?) => {
    ::std::collections::HashMap::from([ $((::std::rc::Rc::<str>::from(stringify!($key)), $value)),* ])
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
