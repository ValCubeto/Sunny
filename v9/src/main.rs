mod colors;
mod about;
mod macros;
mod aliases;
mod values;
mod context;
mod args;
mod commands;

use args::parse_args;

pub fn main() {
  parse_args();
}
