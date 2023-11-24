use crate::{
  types::{Pointer, VariantMap, StringPtr },
  instances::Instance
};

struct Enum {
  name: StringPtr,
  variants: VariantMap
}

type EnumPtr = Pointer<Enum>;

struct Variant {
  prototype: EnumPtr,
  variant_id: usize,
  value: Instance
}
