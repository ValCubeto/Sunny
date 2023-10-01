/// Create a HashMap in a cooler way using the arrow syntax
#[macro_export]
macro_rules! hashmap {
  () => {
    ::std::collections::HashMap::new()
  };
  ($($key: expr => $value: expr),* $(,)?) => {
    ::std::collections::HashMap::from([
      $(
        ($key, $value)
      ),*
    ])
  };
}

#[macro_export]
macro_rules! error {
  ($name: expr, ) => {
    todo!()
  };
}