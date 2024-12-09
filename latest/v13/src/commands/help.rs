use crate::strings::{ LANG, VERSION };
use crate::terminal::Stylize;

pub fn help() {
  println!("{} v{VERSION}", LANG.bold());
}
