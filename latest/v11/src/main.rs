#[allow(unused)]
#[macro_use]
pub mod errors;

pub mod parse;
pub mod colors;
pub mod types;

#[cfg(test)]
mod tests;

// TODO: custom `unexpected` function that tries to print the type of the value
// Examples:
// `unexpected('1')` -> `Syntax error: unexpected number`
// `unexpected('i')` -> `Syntax error: unexpected \`if\` keyword here`

// HINT: structure of the scopes
// ```rs
// typedef Scopes = Vec<hashbrown::HashMap<
//   Rc<str>, /* shared identifier */
//   ItemType /* indicating where to search the actual value */
// >
// let constants: hashbrown::HashMap<Rc<str>, Constant> = HashMap::new();
// ```
// Example:
// if an `ItemType::Const` is found, search the value in `constants`
//
// This allows faster searches, but obviously uses more memory

fn main() {
  println!("r: {}", 2 * 5 | 7);
  let file_name = "files/main.sny";

  let code = std::fs::read_to_string(file_name)
    .expect("failed to read the file");
  // [
  //   {
  //     "std" => ?
  //     "int" => BuiltInType::Int8
  //   },
  //   {
  //     "TEST" => Constant { ty: Int32, val: Value::u8(1) }
  //   }
  // ]
  parse::parse_file(file_name, &code);
}

// Global variables:
// - std
// - use std::numbers::{ u8, u16, u32, u64, i8, i16, i32, i64, f32, f64 };
// - typedef int = i32
// - typedef uint = u32
// - typedef float = f32
// - use std::lists::Vec
// - use std::display::String
// - use std::io::println
// - use std::process::exit
// - use std::debug::assert!
// - use std::result::Result::{ self, Ok, Err };
// - use std::option::Option::{ self, Some, None };

// Std lib structure:
// - mem:
//   - psize
//   - alloc (malloc)
//   - alloc_init (calloc)
//   - realloc
//   - free
//   - copy
//   - move
//   - Error
//   - Result
// - cmp:
//   - Eq, Neq
//   - LessThan, GreaterThan
//   - LessThanOrEq, GreaterThanOrEq
// - ops:
//   - Add, Sub, Mul, Div, Pow, Mod
//   - Not
//   - And, Or
//   - BinAnd, BinOr, BinXor
// - numbers:
//   - Number
//   - u8, u16, u32, u64, u128, usize
//   - i8, i16, i32, i64, i128, isize
//   - f32, f64, f128
//   - BigInt, BigUint, BigDecimal
//   - Complex, Ratio, BigRational
// - lists:
//   - Vec
// - display:
//   - ToString
//   - FromString
//   - Display
//   - String
//   - Char
// - fmt:
//   - Display, Debug, NumberFormat
// - io:
//   - stdin
//   - readln
//   - stdout
//   - print
//   - println
//   - stderr
//   - eprint
//   - eprintln
//   - ask
//   - Error
//   - Result
// - debug:
//   - assert!
//   - BOLD, BOLD_END, ITALIC, ITALIC_END, ...
//   - RED, GREEN, BLUE, YELLOW, ...
//   - COLOR_END
//   - Stylize
// - net:
// - regex:
//   - Regex
// - time:
//   - sleep
//   - Instant
//   - Duration
//   - IntoDuration (5.seconds(), etc)
// - os:
//   - syscall
//   - EOL
//   - ARCH
// - fs:
//   - File
//   - Dir
//   - read_file
//   - write_file
// - option:
//   - Option
// - result:
//   - Result
// - convert:
//   - From, Into
//   - TryFrom, TryInto
// - math:
//   - PI
//   - E
//   - trait Math
// - ranges:
//   - Range
// - env:
//   - get_var
//   - use_var!
// - json:
//   - json!
//   - jsonify
//   - parse_json
//
// - cfg
// - matches!
// - stringify!
// - type_name!
// - type_name_of!
//
// - bool
// - Function
// - HashMap (secure, std)
// - HashMap (fast, hashbrown::HashMap)
// - Dict (BTreeMap)
