#[macro_use]
  pub mod errors;
pub mod parse;
pub mod colors;

fn main() {
  let file_name = "main";
  let code = "\t\n
    const A = 1
  ";
  parse::parse_file(file_name, code);
}

// Global variables:
// - std
// - use std::numbers::{ u8, u16, u32, u64, i8, i16, i32, i64, f32, f64 };
// - typedef int = i32
// - typedef uint = u32
// - typedef float = f32
// - use std::lists::Vec
// - use std::strings::String
// - use std::io::println
// - use std::process::exit
// - use std::debug::assert!

// Std lib structure:
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
// - vecs:
//   - Vec
// - strings:
//   - ToString
//   - String
//   - Char
// - fmt:
//   - Display, Debug
// - io:
//   - stdin
//   - readln
//   - stdout
//   - print
//   - println
//   - stderr
//   - eprint
//   - eprintln
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
//   - MathOps (sqrt, cbrt, log2, log10, ...)
// - ranges:
//   - Range
// - env:
//   - get_var
//   - use_var!
// - json:
//   - json!
//
// - cfg!
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
