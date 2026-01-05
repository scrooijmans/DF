# Type Alias Date64Array Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/array/primitive_array.rs.html#328" class="src">Source</a>

``` rust
pub type Date64Array = PrimitiveArray<Date64Type>;
```

Expand description

A [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray") of milliseconds since UNIX epoch stored as `i64`

This type is similar to the [`chrono::NaiveDate`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/date/struct.NaiveDate.html "struct chrono::naive::date::NaiveDate") type and can hold values such as `2018-11-13`

## Aliased Type<a href="https://docs.rs/arrow/latest/arrow/array/array/type.Date64Array.html#aliased-type" class="anchor">§</a>

``` rust
pub struct Date64Array { /* private fields */ }
```
