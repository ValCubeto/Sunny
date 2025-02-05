#![allow(unused)]

pub const LANG: &str = "Sunny";
pub const VERSION: &str = "0.13";
pub const EXTENSION: &str = "sny";

#[macro_export]
macro_rules! path {
  () => {
    format!("{}:{}:{}", file!(), line!(), column!())
  };
}
