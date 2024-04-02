
#[allow(unused)]
mod colors;

macro_rules! printf {
  ($($arg: expr),*) => {{
    use ::std::io::Write;
    print!($($arg),*);
    ::std::io::stdout().flush().unwrap();
  }};
}

fn main() {
  println!("Welcome to REPL v0.1!");
  println!("Call {} for more information", "help()");
  println!();
  printf!("> ");
}
