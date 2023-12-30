// parse tokens
// ["fun", "main", "(", ")", "{", "let", "text", "=", "'hello'", "}"]

// make an AST
// Fun { name: "main", args: [], body: [Let { name: "text", value: "Hello" }] }

// call "main"

fn main() {
  println!("Hello, world!");
}

// What's a type?
// A 16-bit unsigned integer being an index of the types table.

// What's a string?
// Represented as type + pointer.
// At its location, there is the length, the capacity, and the bytes.

// What's an int?
// Represented as type + a 32-bit integer.
