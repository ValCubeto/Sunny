#[macro_use]
mod errors;
#[macro_use]
mod macros;
#[allow(unused)]
mod terminal;
mod context;
mod modules;
mod values;

use context::Context;
use modules::parse_module_with_name;

fn main() {
  let code = "
    // comment
  ";
  let mut ctx = Context::new(code);
  let _main_mod = parse_module_with_name("main".into(), code);
  // if let Some(Value::Function(main_func)) = main_mod.get("main".into()) {}
}

// use std::collections::LinkedList;
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

// let mut ast: LinkedList<Instruction> = LinkedList::new();
// ast.push_back(Instruction {
//   kind: InstructionType::GetProp,
//   data: Box::new(["a", "b"])
// })

// pub enum InstructionType {
//   /// # The `use` keyword
//   /// `{ path: Path, alias: String }`
//   /// # Example
//   /// `use std::debuggin::debug as dbg`
//   Import,

//   /// # The `::` operator.
//   /// `{ path: String[] }`
//   /// # Example
//   /// `std::terminal::Colorize`
//   GetItem,

//   /// # The `.` operator.
//   /// `{ path: String[] }`
//   GetProp,

//   /// # The `(...)` syntax.
//   /// `{ func: Function, generics: G[], args: A[] }`
//   Call,
// }

// pub struct Instruction {
//   kind: InstructionType,
//   data: Box<dyn std::any::Any>
// }
