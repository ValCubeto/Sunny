use std::io::*;

fn main() -> Result<()> {
  let input: u32 = loop {
    print!("> ");
    stdout().flush()?;
    match read()?.trim().parse() {
      Ok(n) => break n,
      Err(_) => {
        println!("Invalid input");
        println!();
      },
    }
  };
  test("Test 1", input, |n| {n});
  Ok(())
}

fn read() -> Result<String> {
  let mut buf = String::new();
  stdin().read_line(&mut buf)?;
  Ok(buf)
}

fn test<N: Into<f64> + Clone>(desc: &str, n: N, f: fn(N) -> N) {
  println!("{desc:?}");
  let sqrt: f64 = n.clone().into().sqrt();
  let result = f(n);
}
