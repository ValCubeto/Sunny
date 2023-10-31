# The Sunny programming language

## What is Sunny?
Sunny is a multiparadigm program, including OOP and functional programming.
Este lenguaje intenta tomar las mejores cosas de muchos lenguajes, incluyendo bastante control sobre cada aspecto, pero sin perder la simpleza del alto nivel.

## No ando muy inspirado como para escribir más

## Getting started
```kotlin
fun main() {
  println('Hello, world!')
}
```

### Functions
```kotlin
fun foo<T>(bar: T)
where
  T: Bar
{
  bar.bar_stuff()
}
```

### Classes and structs
```rust
class Foo {
  struct {
    bar: Bar
  }
  pub fun new(bar: _) -> Self {
    Self { bar }
  }
  priv fun secret() {
    println('Boo!')
  }
  impl Debug { }
  fun ToString::to_string(self) {
    return "I'm a ${Self.name} with bar: ${self.bar}"
  }
}
```

### Anonymous functions
```rust
let add = /a, b/ { a + b }
```
