# Macro downcast_temporal_array Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#306" class="src">Source</a>

``` rust
macro_rules! downcast_temporal_array {
    ($values:ident => $e:expr, $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
    (($($values:ident),+) => $e:expr, $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
    ($($values:ident),+ => $e:block $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
    (($($values:ident),+) => $e:block $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
}
```

Expand description

Downcast an [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") to a temporal [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") based on its [`DataType`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html "enum datafusion::common::arrow::datatypes::DataType") accepts a number of subsequent patterns to match the data type

``` rust

fn print_temporal(array: &dyn Array) {
    downcast_temporal_array!(
        array => {
            for v in array {
                println!("{:?}", v);
            }
        }
        DataType::Utf8 => {
            for v in as_string_array(array) {
                println!("{:?}", v);
            }
        }
        // You can also add a guard to the pattern
        DataType::LargeUtf8 if true => {
            for v in as_largestring_array(array) {
                println!("{:?}", v);
            }
        }
        t => println!("Unsupported datatype {}", t)
    )
}
```
