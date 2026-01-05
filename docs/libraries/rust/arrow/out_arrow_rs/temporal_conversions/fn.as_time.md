# Function as_time Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#286" class="src">Source</a>

``` rust
pub fn as_time<T>(v: i64) -> Option<NaiveTime>where
    T: ArrowPrimitiveType,
```

Expand description

Converts an [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType") to [`NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime")
