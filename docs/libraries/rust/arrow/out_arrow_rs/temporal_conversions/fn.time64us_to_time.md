# Function time64us_to_time Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#94" class="src">Source</a>

``` rust
pub fn time64us_to_time(v: i64) -> Option<NaiveTime>
```

Expand description

converts a `i64` representing a `time64(us)` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")
