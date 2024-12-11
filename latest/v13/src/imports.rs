#[macro_export]
macro_rules! use_item {
  ($($cmd:ident),*) => {
    $(
      mod $cmd;
      use $cmd::{$cmd};
    )*
  };
}
