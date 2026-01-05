# Function as_duration Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#310" class="src">Source</a>

``` rust
pub fn as_duration<T>(v: i64) -> Option<TimeDelta>where
    T: ArrowPrimitiveType,
```

Expand description

Converts an [`ArrowPrimitiveType`](https://docs.rs/arrow/latest/arrow/array/trait.ArrowPrimitiveType.html "trait arrow::array::ArrowPrimitiveType") to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")
