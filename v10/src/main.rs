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
//     class bool
//     class i32    // std::number::i32
//     typedef int = i32
//     class String
//     fun println(data: String)    // data: Display
//     fun eprintln(data: String)
//     fun assert(condition: bool) {  if !condition { quit("Assertion failed") }  }
//     fun quit(reason: String) {  eprintln(reason); exit(1)  }
// }

// INSTANT GARBAGE COLLECTION
// Keep values like function names only if they're used,
// replace identifiers/aliases/paths with the actual values,
// make functions inline if they're only an expression
// + borrow checker: delete things when no one's using them

fn main() {
  // 
}
