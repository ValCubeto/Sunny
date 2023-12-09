mod value;
use std::{
  mem::{
    size_of_val as size_by_val,
    // size_of
  },
  slice::from_raw_parts as read_bytes,
  any::type_name
};

#[allow(clippy::size_of_ref)]
fn main() {
  // size of str is not known at compile time... but size of slice
  let input = "Hello \u{FF}";

  // let len = size_by_val(&input);
  let len = input.len();
  let ptr = input as *const str as *const u8 as usize;
  let ptr_len = size_by_val(&ptr);

  println!("Pointer value  : 0x{:X}", ptr);
  println!("Size of pointer: {} bytes | {} bits", ptr_len, ptr_len * 8);
  println!();
  println!("Value of input : {:?}", input);
  println!("Type of input  : {}", type_name_of(&input));
  println!("Size of input  : {} bytes | {} bits", len, len * 8);
  println!();
  unsafe {
    let bytes = read_bytes(ptr as *const u8, len);
    let max_index_len = bytes.len().to_string().len();
    // Bytes reversed because of little-endian!
    for (i, &byte) in bytes.iter().enumerate() {
      println!("Byte #{:<0len$}: {:08b}", i + 1, byte, len = max_index_len);
      println!("{:?}", byte as char);
    }
  };
}

pub fn ptr_to_first_byte<T>(ptr: &T) -> usize {
  ptr as *const T as usize
}

#[inline(always)]
pub fn type_name_of<T>(_value: &T) -> &str {
  type_name::<T>()
}
