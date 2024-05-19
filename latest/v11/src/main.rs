#[allow(unused)]
#[macro_use]
pub mod errors;

pub mod parse;
pub mod colors;
pub mod lang;

#[cfg(test)]
mod tests;

// TODO: custom `unexpected` function that tries to print the type of the value
// Examples:
// `parser.unexpected('1')` -> `Syntax error: unexpected number`
// `parser.unexpected('i')` -> `Syntax error: unexpected \`if\` keyword here`

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
  let file_name = "files/main.sny";

  let code = std::fs::read_to_string(file_name)
    .expect("failed to read the file")
    // .trim() removes some characters that are considered
    // invalid in a Sunny file
    .trim_matches(|c| matches!(c, ' ' | '\t' | '\n' | '\r'));

  if code.is_empty() {
    return;
  }

  // std is a keyword instead of a reference to itself to
  // avoid infinite recursion (`std::std::std::...`)
  // In the case of `true`, `false`, `self`, `Self`,
  // `super`, `Super`, etc. they're keywords to prevent overwrites.
  // for example, `const true = 1` won't be allowed.

  // [
  //   {
  //     "Int8" => BuiltInType::Int8
  //   },
  //   {
  //     "TEST" => Constant { ty: Int32, val: Value::u8(1) }
  //   }
  // ]
  parse::parse_file(file_name, code);
}

// Global variables:
// - Vec
// - String
// - println
// - eprintln
// - quit
// - panic
// - assert!
// - Result::{ self, Ok, Err };
// - Option::{ self, Some, None };

// Std lib structure:
// - mem:
//   - PSize
//   - alloc (malloc)
//   - alloc_init (calloc)
//   - realloc
//   - free
//   - copy (memcpy)
//   - move
//   - Error
//   - Result
// - cmp:
//   - Cmp
//   - Eq
// - ops:
//   - Add, Sub, Mul, Div, Pow, Mod
//   - Not
//   - And, Or
//   - BinAnd, BinOr, BinXor
// - numbers:
//   - Number (generic number trait)
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
