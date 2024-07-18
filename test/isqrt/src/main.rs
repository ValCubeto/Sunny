use std::io::*;
use std::time::Instant;

type N = u64;

fn main() -> Result<()> {
  let input: N = loop {
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

  println!("{}, {}, {}", N::MAX, N::MAX as f64, f64::MAX);

  test("Test 1: underestimate (r <= f(n))", input, |n| {
    let mut l = 0;
    while (l + 1) * (l + 1) <= n {
      l += 1;
    }
    l
  });

  // slowest
  test("Test 2: overestimate (f(n) <= r)", input, |n| {
    let mut r = n;
    while r * r > n {
      r -= 1;
    }
    r
  });

  test("Test 3: linear search (asc) using addition", input, |n| {
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

  test("Test 4: binary search", input, |n| {
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

  test("Test 5: Heron's method", input, |n| {
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
  test("Test 6: recursion with bitwise ops", input, recur_isqrt);

  test("Test 7: iter with bitwise ops", input, |n| {
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

  test("Test 8: conversion + floor", input, |n| {
    (n as f64).sqrt().floor() as N
  });

  Ok(())
}

fn read() -> Result<String> {
  let mut buf = String::new();
  stdin().read_line(&mut buf)?;
  Ok(buf)
}

fn test(desc: &str, n: N, f: fn(N) -> N) {
  let sqrt: f64 = (n as f64).sqrt();

  println!("{desc:?}");

  let runs = 10_000;

  let mut sum = 0;
  for _ in 0..runs {
    let start = Instant::now();
    let _ = f(n);
    sum += start.elapsed().as_nanos();
  }
  let total = (sum / runs) as f64 / 1e5;
  let result = f(n);
  println!("n.sqrt() = {sqrt:.5}");
  println!("isqrt({n}) = {result}");
  println!("Average: {total:.5} ms");
  println!();
}
