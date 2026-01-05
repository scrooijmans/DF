# Macro downcast_integer_array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/cast.rs.html#140" class="src">Source</a>

``` rust
macro_rules! downcast_integer_array {
    ($values:ident => $e:expr, $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
    (($($values:ident),+) => $e:expr, $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
    ($($values:ident),+ => $e:block $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
    (($($values:ident),+) => $e:block $($p:pat $(if $pred:expr)* => $fallback:expr $(,)*)*) => { ... };
}
```

Expand description

Given one or more expressions evaluating to an integer [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") invokes the provided macro with the corresponding array, along with match statements for any non integer array types

``` rust

fn print_integer(array: &dyn Array) {
    downcast_integer_array!(
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
