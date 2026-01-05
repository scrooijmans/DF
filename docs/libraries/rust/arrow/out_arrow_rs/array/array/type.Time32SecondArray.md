# Type Alias Time32SecondArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#334" class="src">Source</a>

``` rust
pub type Time32SecondArray = PrimitiveArray<Time32SecondType>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of seconds since midnight stored as `i32`

This type is similar to the [`chrono::NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") type and can hold values such as `00:02:00`

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Time32SecondArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Time32SecondArray { /* private fields */ }
```
