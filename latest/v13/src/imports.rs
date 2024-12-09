
#[macro_export]
macro_rules! use_fn {
  ($($cmd:ident),*) => {
    $(
      mod $cmd;
      use $cmd::{$cmd};
    )*
  };
}
