extern crate num_bigint;
extern crate bigdecimal;

mod colors;
mod about;
mod macros;
mod aliases;
mod values;
mod context;
mod args;
mod commands;
mod table;

use std::str::FromStr as _;
use args::parse_args;
use bigdecimal::BigDecimal;
use num_bigint::BigInt;

pub fn main() {
  let a = &BigDecimal::from_str("0.1").unwrap();
  let b = &BigDecimal::from_str("0.2").unwrap();
  let c = &BigDecimal::from_str("0.3").unwrap();
  println!("{} + {} = {}; {} == {} = {}",
    a, b, a + b,
    a + b, c, a + b == *c
  );
  println!("{}", BigInt::from_str(i64 ::MAX.to_string().as_str()).unwrap() + BigInt::from_str("100000").unwrap());
  parse_args();
}
