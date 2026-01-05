# Function date32_to_datetime Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#55" class="src">Source</a>

``` rust
pub fn date32_to_datetime(v: i32) -> Option<NaiveDateTime>
```

Expand description

converts a `i32` representing a `date32` to [`NaiveDateTime`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/naive/datetime/struct.NaiveDateTime.html "struct chrono::naive::datetime::NaiveDateTime")
