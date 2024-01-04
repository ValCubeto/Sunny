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

// Global
// {
//     mod std {
//         mod number {
//             class i32
//         }
//         mod process {
//             fun print(data: Display) {
//                 stdout().write(data.display()).flush()
//             }
//             fun eprint(data: Display) {
//                 stderr().write(data.display()).flush()
//             }
//             fun println(data: Display) {
//                 print(f"{data}{os::EOL}")
//             }
//             fun readln() {
//                 stdin().read_until(os::EOL)
//             }
//         }
//     }
//     class bool {
//         fun switch() -> Self {
//             self = !self
//             return self
//         }
//     }
//     typedef int = std::number::i32
//     class String
//     fun assert(condition: bool) {  if !condition { quit("Assertion failed") }  }
//     fun quit(reason: String) {  eprintln(reason); exit(1)  }
// }

// INSTANT GARBAGE COLLECTION
// Keep values only while they're used,
// replace identifiers/aliases/paths with the actual values,
// inline functions if they're only an expression

// AST
// A liked list, with each element including sized metadata and unsized data
// The list is just a pointer to the first and last element
// Each element has a pointer to the next element and a size, then the actual data
// The data must include the action

// DATA TYPES
// Structs are only syntax sugar for tuples, as they're for primitive values
// u8 ... u64 and its signed variants
// strings (ptr -> (size, capacity, ...data))
// vectors (ptr -> (size, capacity, ?elem_size, ...data))
// booleans

fn main() {
  let ast = Ast::new();
  let mut item = ast.first as *const AstItem;
  while let Some(item) = unsafe {*item}.next {
    item = item
  }
}

struct Ast {
  first: usize,
  last: usize
}
impl Ast {
  pub fn new() -> Self {
    let ptr = &AstItem { next: None } as *const _ as usize;
    Ast {
      first: ptr,
      last: ptr
    }
  }
  pub fn push(&mut self, ptr: &AstItem) {
    self.last = ptr as *const _ as usize
  }
  pub fn consume(&mut self) -> AstItem {
    let item = unsafe { *(self.first as *const AstItem) };
    self.first = item.next.expect("");
    item
  }
}

#[derive(Clone, Copy)]
struct AstItem {
  next: Option<usize>
}
