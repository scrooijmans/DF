# Type Alias Time64MicrosecondArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#346" class="src">Source</a>

``` rust
pub type Time64MicrosecondArray = PrimitiveArray<Time64MicrosecondType>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of microseconds since midnight stored as `i64`

This type is similar to the [`chrono::NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") type and can hold values such as `00:02:00.123456`

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/type.Time64MicrosecondArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Time64MicrosecondArray { /* private fields */ }
```
