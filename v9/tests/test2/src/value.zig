const std = @import("std");

const Value = struct {
  // descriptor: usize,
  ptr: usize,
  len: usize
};

// every second i spend programming in zig
// i think it's better javascript more than better C
pub fn main() void {
  const my_uint8: u8 = 52;
  const u8_value = Value {
    .ptr = &my_uint8,
    .len = @sizeOf(u8)
  };

  const my_cstring: [*](u8) = "Hello!";
  const cstring_value = Value {
    .ptr = &my_cstring,
    .len = @sizeOf(@TypeOf(my_cstring))
  };

  _ = u8_value;
  _ = cstring_value;
}
