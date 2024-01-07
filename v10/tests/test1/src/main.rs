use dlopen::raw::Library;

fn main() {
  let (_lib, hello) = load_func("hello");
  println!("'hello' loaded");
  hello();
}

fn load_func(name: &str) -> (Library, fn()) {
  let lib = Library::open(format!("../../lib/bin/{}.dll", name))
    .unwrap_or_else(|err| panic!("failed to open '{name}.dll': {err}"));
  unsafe {
    let func = lib.symbol(name)
      .unwrap_or_else(|err| panic!("no '{name}' function: {err}"));
    (lib, func)
  }
}
