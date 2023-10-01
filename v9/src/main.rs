mod macros;
mod run;
mod repl;

pub fn main() {
  println!("Hello, world!");
  let ages = hashmap! {
    "hello" => "world",
    "asd" => "1",
  };
  println!("ages = {ages:?}");
}
