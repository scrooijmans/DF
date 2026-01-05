# Function time_to_time32s Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#117" class="src">Source</a>

``` rust
pub fn time_to_time32s(v: NaiveTime) -> i32
```

Expand description

converts [`NaiveTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/time/struct.NaiveTime.html "struct chrono::naive::time::NaiveTime") to a `i32` representing a `time32(s)`
