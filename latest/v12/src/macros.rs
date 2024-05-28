macro_rules! parser_method {
  (fn $name:ident($($arg_name:ident: $arg_type:ty),*) -> $out:ty) => {
    impl<'a> Parser<'a> {
      #[inline(always)]
      pub fn $name(&mut self, $($arg_name: $arg_type),*) -> $out {
        $name(self, $($arg_name),*)
      }
    }
  };
}
