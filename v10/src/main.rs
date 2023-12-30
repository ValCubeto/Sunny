// parse tokens
// ["fun", "main", "(", ")", "{", "let", "text", "=", "'hello'", "}"]

// make an AST
// Fun { name: "main", args: [], body: [Let { name: "text", value: "Hello" }] }

// call "main"

fn main() {
  println!("Hello, world!");
}

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
pub struct Value {
  type_index: usize,
  value: usize
}

use once_cell::unsync::Lazy;
use std::sync::Mutex;
use hashbrown::HashMap;
use std::rc::Rc;

thread_local! {
  #[allow(non_upper_case_globals)]
  pub static global: Lazy<Mutex<HashMap<Rc<str>, Value>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
  });
}

fn test() {
  global.with(|g| {
    let mut g_lock = g.lock().expect("global was borrowed elsewhere");
    g_lock.insert("hola".into(), Value { type_index: 0, value: 0 });
  });
}
// static types: Lazy<Mutex<Vec<fn(usize)>>> = Lazy::new
