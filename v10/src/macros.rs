macro_rules! wrapper_enum {
  (
    enum$name:ident{$($variant:ident$(<$($gen:ty),*>)?),*$(,)?}
  ) => {
    enum $name {
      $( $variant($variant$(<$( $gen ),*>)?) ),*
    }
  }
}
