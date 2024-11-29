extern crate argmap;

fn main() {
  // NOTE: args[0] is the name of the executable
  let (args, flags) = argmap::parse(std::env::args());
  println!("args = {:?}", args);
  println!("flags = {:?}", flags);
}
