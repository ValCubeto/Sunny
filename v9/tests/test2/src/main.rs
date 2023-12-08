mod value;
use std::{mem::{
  size_of_val as size_by_val,
  size_of
}, rc::Rc};

#[allow(clippy::size_of_ref)]
fn main() {
  let u8_val = 99_u8;
  let usize_val = 105_usize;
  let string = "hello";

  println!("0x{:x}: {} bytes | {} bits", ptr_as_usize(&u8_val), size_by_val(&&u8_val), size_by_val(&&u8_val) * 8);
  println!("0x{:x}: {} bytes | {} bits", ptr_as_usize(&usize_val), size_by_val(&&usize_val), size_by_val(&&usize_val) * 8);
  println!("0x{:x}: {} bytes | {} bits", ptr_as_usize(&string), size_of::<Rc<str>>(), size_of::<Rc<str>>() * 8);
  println!("{} bits", size_by_val(&Box::new(1_usize)) * 8);
}

pub fn ptr_as_usize<T>(ptr: &T) -> usize {
  ptr as *const T as usize
}
