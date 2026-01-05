# Macro downcast_integer Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#76" class="src">Source</a>

``` rust
macro_rules! downcast_integer {
    ($($data_type:expr),+ => ($m:path $(, $args:tt)*), $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
}
```

Expand description

Given one or more expressions evaluating to an integer [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") invokes the provided macro `m` with the corresponding integer [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType"), followed by any additional arguments

``` rust

macro_rules! dictionary_key_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn dictionary_key_size(t: &DataType) -> u8 {
    match t {
        DataType::Dictionary(k, _) => downcast_integer! {
            k.as_ref() => (dictionary_key_size_helper, u8),
            _ => unreachable!(),
        },
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX,
    }
}

assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::Int32), Box::new(DataType::Utf8))), 4);
assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::Int64), Box::new(DataType::Utf8))), 8);
assert_eq!(dictionary_key_size(&DataType::Dictionary(Box::new(DataType::UInt16), Box::new(DataType::Utf8))), 2);
```
