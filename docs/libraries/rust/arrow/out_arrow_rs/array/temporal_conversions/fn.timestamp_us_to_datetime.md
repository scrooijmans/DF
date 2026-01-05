# Function timestamp_us_to_datetime Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#188" class="src">Source</a>

``` rust
pub fn timestamp_us_to_datetime(v: i64) -> Option<NaiveDateTime>
```

Expand description

converts a `i64` representing a `timestamp(us)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")
