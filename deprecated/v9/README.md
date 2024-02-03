# The Sunny programming language

## What is Sunny?
Sunny is a multiparadigm program, including OOP and functional programming.
Se intenta simplificar al máximo la escritura de código para que no tengas que
preocuparte de lo que está pasando por detrás, pero también, si quieres,
puedas escribir este comportamiento tú mismo.

## No ando muy inspirado como para escribir más

## Getting started
### Installing Sunny
-
### Creating your first program
```kotlin
# main.sny
fun main() {
  println('Hello, world!')
}
```
### Running your program
```bash
sunny run main
```

## Features
- Strong typing
- Models
- Typed values
- Implicit types
- Destructuring

## Functions
The types cannot be changed. Instead, you can pass your own type
via generic parameters and use these types as a parameter type or return type
```kotlin
fun foo<T>(bar: T)
```
This specifies that the type `T` must implement the `Bar` model.
This is useful if you need to use a method of this model,
in this case, `display`.
The `where` keyword is just sugar when specifying implementations,
you can put them into the generic declaration, but it's needed
if you need to specify other types' implementations,
like `String: From<T>`. If you put it in the generic declaration,
you will receive a generic parameter called `String`, and this is
not we wanted.
The `impl` keyword is also sugar, it's just to be more clear.
```rust
where
  T: impl Display
```
Generic types are structs, and structs are values,
so you can do some operations with them.
```python
{
  if T == String {
    println('T is String!')
  }
  println(bar.display())
}
```

## Classes and structs
A class is like the impl block in Rust
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
  impl Debug
  fun ToString::to_string(self) {
    return "I'm a ${Self.name} with bar: ${self.bar}"
  }
}
```

## Anonymous functions
```rust
let add = /a, b/ a + b
let sub = /a, b/ {
  return a - b
}
```
