extern crate rand;
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

use args::parse_args;
use rand::Rng;

pub fn main() {
  let mut thread = rand::thread_rng();
  debug!(thread.gen_range(0..=100));
  parse_args();
}
