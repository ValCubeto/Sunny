extern crate rand;
extern crate num_bigint;
extern crate bigdecimal;

extern crate peekmore;

extern crate chrono;

extern crate tokio;
extern crate hyper;

extern crate regex;
extern crate serde;

extern crate bincode;
extern crate dlopen;

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

fn main() {
  parse_args();
}
