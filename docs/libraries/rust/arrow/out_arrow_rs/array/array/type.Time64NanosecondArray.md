# Type Alias Time64NanosecondArray Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#352" class="src">Source</a>

``` rust
pub type Time64NanosecondArray = PrimitiveArray<Time64NanosecondType>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of nanoseconds since midnight stored as `i64`

This type is similar to the [`chrono::NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") type and can hold values such as `00:02:00.123456789`

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Time64NanosecondArray.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Time64NanosecondArray { /* private fields */ }
```
