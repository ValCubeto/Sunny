// parse tokens
// ["fun", "main", "(", ")", "{", "let", "text", "=", "'hello'", "}"]

// make an AST
// Fun { name: "main", args: [], body: [Let { name: "text", value: "Hello" }] }

// call "main"

// # DEFINITIONS
// What's a type?
// An index in the types table.

// What's the types table?
// A vec of functions describing how to read a value.

// What's an int?
// A 32-bit integer.

// What's a string?
// A pointer to a length, a capacity, and the actual chars.

// What's a vec?
// A pointer to a length, capacity, item size, and actual items.

// Each value has its own length, but to store them in global
// they need some padding to be all the same size.
// The max size is the size of a pointer.
// If any value is larger, use a pointer instead.

// Global
// {
//     class bool
//     class i32    // std::number::i32
//     typedef int = i32
//     class String
//     fun println(data: String)    // data: Display
//     fun eprintln(data: String)
//     fun assert(condition: bool) {  if !condition { quit("Assertion failed") }  }
//     fun quit(reason: String) {  eprintln(reason); exit(1)  }
// }

pub struct Class<'a> {
  name: &'a str,
  inner: ClassInner
}
pub enum ClassInner {
  BuiltIn {
    byte_size: u8
  },
  Defined {}
}

use hashbrown::HashMap;
use std::rc::Rc;

fn main() {
  println!("Hello, world!");
  let mut ctx = GlobalContext::new();
}

// Wrapper to be used in maps.
pub struct Value {
  pub type_index: usize,
  pub ptr: usize
}

pub struct GlobalContext {
  pub types: Vec<fn(usize)>,
  pub global: HashMap<Rc<str>, Value>,
}

#[allow(clippy::new_without_default)]
impl GlobalContext {
  pub fn new() -> Self {
    GlobalContext {
      types: Vec::new(),
      global: HashMap::new(),
    }
  }
}
