use std::{collections::HashMap, io::Write};

use dlopen::raw::Library;

fn main() {
  let mut libs: HashMap<&str, Library> = HashMap::new();
  let mut functions: HashMap<&str, fn()> = HashMap::new();
  loop {
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();
    match line {
      "hello" => {
        let lib = libs.entry("hello").or_insert_with(|| {
          println!("Loading 'hello.dll'");
          Library::open("../../lib/bin/hello.dll")
            .unwrap_or_else(|err| panic!("failed to open 'hello.dll': {}", err))
        });
        let func = functions.entry("hello").or_insert_with(|| unsafe {
          println!("Loading 'hello' from 'hello.dll'");
          lib.symbol("hello")
            .unwrap_or_else(|err| panic!("no function: {}", err))
       });
       func();
      },
      "bye" => {
        let lib = libs.entry("bye").or_insert_with(|| {
          println!("Loading 'bye.dll'");
          Library::open("../../lib/bin/bye.dll")
          .unwrap_or_else(|err| panic!("failed to open 'bye.dll': {}", err))
        });
        let func = functions.entry("bye").or_insert_with(|| unsafe {
          println!("Loading 'bye' from 'bye.dll'");
          lib.symbol("bye")
            .unwrap_or_else(|err| panic!("no function: {}", err))
       });
       func();
      },
      _ => continue
    }
    println!("libraries: {:?}", libs.keys().collect::<Vec<_>>());
    println!("libraries: {:?}", functions.keys().collect::<Vec<_>>());
  }
}
