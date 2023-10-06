use std::io::{ stdin, Write as _ };

#[allow(unused)]
pub fn main() {
  loop {
    let input = stdin()
      .read_line();
    let line = input.trim();
    debug!(line);
  }
  todo!();
}
