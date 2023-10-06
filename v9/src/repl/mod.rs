use std::io::{ stdout, stdin, Write as _ };
use crate::about::NAME;

#[allow(unused)]
pub fn main() {
  loop {
    print!("{NAME}> ");

    // otherwise the program reads the line
    // and THEN prints the prompt
    stdout()
      .flush()
      .expect("failed to print the prompt");

    let mut buf: String = String::new();
    stdin()
      .read_line(&mut buf)
      .expect("failed to read the line");

    // I use String because I need
    // to index later
    let line: String = buf
      .trim()
      .to_string();

    // no longer needed
    drop(buf);

    if line.is_empty() {
      continue;
    }

    debug!(line);
    todo!();
  }
}
