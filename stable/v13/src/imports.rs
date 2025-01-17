#[macro_export]
macro_rules! use_item {
  ($($cmd:ident),*) => {
    $(
      pub mod $cmd;
      use $cmd::{$cmd};
    )*
  };
}
