# Macro downcast_run_end_index Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#191" class="src">Source</a>

``` rust
macro_rules! downcast_run_end_index {
    ($($data_type:expr),+ => ($m:path $(, $args:tt)*), $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
}
```

Expand description

Given one or more expressions evaluating to an integer [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html "enum arrow::datatypes::DataType") invokes the provided macro `m` with the corresponding integer [`RunEndIndexType`](https://docs.rs/arrow/latest/arrow/datatypes/trait.RunEndIndexType.html "trait arrow::datatypes::RunEndIndexType"), followed by any additional arguments

``` rust

macro_rules! run_end_size_helper {
  ($t:ty, $o:ty) => {
      std::mem::size_of::<<$t as ArrowPrimitiveType>::Native>() as $o
  };
}

fn run_end_index_size(t: &DataType) -> u8 {
    match t {
        DataType::RunEndEncoded(k, _) => downcast_run_end_index! {
            k.data_type() => (run_end_size_helper, u8),
            _ => unreachable!(),
        },
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => u8::MAX,
        _ => u8::MAX,
    }
}

assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int32, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 4);
assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int64, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 8);
assert_eq!(run_end_index_size(&DataType::RunEndEncoded(Arc::new(Field::new("a", DataType::Int16, false)), Arc::new(Field::new("b", DataType::Utf8, true)))), 2);
```
