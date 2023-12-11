mod value;
use std::{
  mem::size_of_val as size_by_val,
  slice::from_raw_parts as read_bytes,
  any::type_name
};

pub type Byte = u8;

#[allow(clippy::size_of_ref)]
fn main() {
  // size of str is not known at compile time... but size of slice
  let input = 255;

  let len = size_by_val(&input);
  // let len = input.len();
  let ptr = ptr_to_first_byte(&input);
  let ptr_len = size_by_val(&ptr);

  println!("Pointer value  : 0x{:X}", ptr);
  println!("Size of pointer: {} bytes | {} bits", ptr_len, ptr_len * 8);
  println!();
  println!("Value of input : {:?}", input);
  println!("Type of input  : {}", type_name_of(&input));
  println!("Size of input  : {} bytes | {} bits", len, len * 8);
  println!();
  unsafe {
    let bytes = read_bytes(ptr as *const Byte, len);
    let max_index_len = bytes.len().to_string().len();
    // With numbers, the bytes are reversed because of little-endian!
    for (i, &byte) in bytes.iter().enumerate() {
      println!("Byte #{:<0len$}: {:08b}", i + 1, byte, len = max_index_len);
      // println!("{:?}", byte as char);
    }
    println!();
    // println!("Back to UTF-8 &str: {:?}", std::str::from_utf8(bytes).expect("invalid utf8"))
  };

  let runtime_ctx = RuntimeContext::default();
}

#[inline(always)]
pub fn ptr_to_first_byte<T: ?Sized>(ptr: &T) -> usize {
  ptr as *const T as *const Byte as usize
}

#[inline(always)]
pub fn type_name_of<T>(_value: &T) -> &str {
  type_name::<T>()
}

struct RuntimeContext {
  types: Box<[fn(Pointer)]>,
  temporal_types: Vec<fn(Pointer)>
}

#[repr(usize)]
#[allow(non_camel_case_types,
        clippy::upper_case_acronyms)]
enum Type {
  PTR,
  BOOL,

  UINT8,
  UINT16,
  UINT32,
  UINT64,
  
  INT8,
  INT16,
  INT32,
  INT64,

  FLOAT32,
  FLOAT64,

  VEC,
  SLICE,

  STRING,    // includes capacity
  STR_SLICE,

  MAP,    // HashMap<K, V> = Map<Hash<K>, V>
}

impl RuntimeContext {
  // SAFETY: as_raw_ptr checks for null pointers
  pub fn default() -> Self {
    let mut types: Vec<fn(Pointer)> = vec![];

    types[Type::BOOL as usize] = |ptr: Pointer| {
      println!("Reading {ptr}");

      unsafe {
        let boolean = read_byte(ptr.as_raw_ptr());
        println!("Got a boolean: {}", boolean != 0);
      };
    };

    types[Type::UINT8 as usize] = |ptr: Pointer| {
      println!("Reading {ptr}");
      unsafe {
        let n = read_byte(ptr.as_raw_ptr());
        println!("Got an u8: {n}")
      }
    };

    types[Type::INT8 as usize] = |ptr: Pointer| {
      println!("Reading {ptr}");
      unsafe {
        let n = read_byte(ptr.as_raw_ptr()) as i8;
        println!("Got an u8: {n}")
      }
    };

    RuntimeContext {
      types: types.into_boxed_slice(),
      temporal_types: Vec::new()
    }
  }
}

/// # Safety
/// This just calls an unsafe function.
/// The indexing shouldn't fail because
/// the len of the byte slice must be 1.
#[inline(always)]
pub unsafe fn read_byte(ptr: *const Byte) -> Byte {
  read_bytes(ptr, 1)[0]
}

struct Pointer(usize);
impl Pointer {
  #[inline(always)]
  pub fn new<T>(ptr: T) -> Self
  where
    usize: From<T>
  {
    Pointer(usize::from(ptr))
  }

  pub fn as_raw_ptr(&self) -> *const Byte {
    let ptr = self.0 as *const Byte;
    assert!(!ptr.is_null());
    ptr
  }

  #[inline(always)]
  pub fn as_raw_ptr_unchecked(&self) -> *const Byte {
    self.0 as *const Byte
  }

  #[inline(always)]
  pub fn as_usize(&self) -> usize {
    self.0
  }
}

impl std::fmt::Display for Pointer {
  #[inline(always)]
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "0x{:x}", self.0)
  }
}
