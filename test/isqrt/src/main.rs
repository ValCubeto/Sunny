use std::io::{ Write, Result, stdin, stdout };
use std::time::Instant;

type N = u64;
const TWO: N = 2;
const MAX_SAFE_INT: N = TWO.pow(63) - 1;

fn prompt(text: &str) {
  print!("{text}");
  stdout().flush().unwrap();
}

fn read() -> Result<String> {
  let mut buf = String::new();
  stdin().read_line(&mut buf)?;
  Ok(buf)
}

fn main() -> Result<()> {
  let input: N = loop {
    prompt("Insert a number to get its integer square root: ");
    match read()?.trim().parse() {
      Ok(n) => break n,
      Err(_) => {
        println!("Invalid input");
        println!();
      },
    }
  };

  println!("Calculating...");
  println!();

  test("underestimate (r <= f(n))", input, |n| {
    let mut l = 0;
    while (l + 1) * (l + 1) <= n {
      l += 1;
    }
    l
  });

  // "overestimate (f(n) <= r)" was omitted because it's too slow

  test("linear search (asc) using addition", input, |n| {
    let mut l = 0;
    let mut a = 1;
    let mut d = 3;
    while a <= n {
      a += d;
      d += 2;
      l += 1;
    }
    l
  });

  test("binary search", input, |n| {
    let mut l = 0;
    let mut r = n + 1;
    while l != r - 1 {
      let m = (l + r) / 2;
      if m * m > n {
        r = m;
      } else {
        l = m;
      }
    }
    l
  });

  test("Heron's method", input, |n| {
    if n < 2 {
      return n;
    }
    let mut x0 = n / 2;
    let mut x1 = (x0 + n / x0) / 2;
    while x1 < x0 {
      x0 = x1;
      x1 = (x0 + n / x0) / 2;
    }
    x0
  });

  fn recur_isqrt(n: N) -> N {
    if n < 2 {
      return n;
    }
    let small_cand = recur_isqrt(n >> 2) << 1;
    let large_cand = small_cand + 1;
    if large_cand * large_cand > n {
      small_cand
    } else {
      large_cand
    }
  }
  // seems to be the fastest, it may use a lot of memory
  test("recursion with bitwise ops", input, recur_isqrt);

  test("iter with bitwise ops", input, |n| {
    if n < 2 {
      return n;
    }
    let mut shift = 2;
    while (n >> shift) != 0 {
      shift += 2;
    }
    let mut result = 0;
    while shift >= 0 {
      result <<= 1;
      let large_cand = result + 1;
      if large_cand * large_cand <= (n >> shift) {
        result = large_cand;
      }
      shift -= 2;
    }
    result
  });

  // fastest (O(1), const time??) but limited by lossy conversion
  test("Test 7: conversion + floor", input, |n| {
    (n as f64).sqrt().floor() as N
  });

  Ok(())
}

static mut TEST_COUNT: u8 = 0;

fn test(desc: &str, n: N, f: fn(N) -> N) {
  if n >= MAX_SAFE_INT {
    panic!("n >= MAX_SAFE_INT");
  }

  // just a counter, why is it unsafe?
  unsafe {
    TEST_COUNT += 1;
    println!("Test No. {TEST_COUNT}: {desc:?}");
  }

  let runs = 10_000;

  let mut sum = 0;
  for _ in 0..runs {
    let start = Instant::now();
    let _ = f(n);
    sum += start.elapsed().as_nanos();
  }
  let total = (sum / runs) as f64 / 1e5;
  let result = f(n);
  println!("isqrt({n}) = {result}");
  println!("Average: {total:.5} ms");
  println!();
}
