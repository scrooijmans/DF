# Macro downcast_primitive Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#355" class="src">Source</a>

``` rust
macro_rules! downcast_primitive {
    ($($data_type:expr),+ => ($m:path $(, $args:tt)*), $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
}
```

Expand description

Given one or more expressions evaluating to primitive [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") invokes the provided macro `m` with the corresponding [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType"), followed by any additional arguments

``` rust

macro_rules! primitive_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn primitive_size(t: &DataType) -> u8 {
    downcast_primitive! {
        t => (primitive_size_helper, u8),
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX
    }
}

assert_eq!(primitive_size(&DataType::Int32), 4);
assert_eq!(primitive_size(&DataType::Int64), 8);
assert_eq!(primitive_size(&DataType::Float16), 2);
assert_eq!(primitive_size(&DataType::Decimal128(38, 10)), 16);
assert_eq!(primitive_size(&DataType::Decimal256(76, 20)), 32);
```
