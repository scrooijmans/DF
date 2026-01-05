# Function timestamp_s_to_time Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#160" class="src">Source</a>

``` rust
pub fn timestamp_s_to_time(secs: i64) -> Option<NaiveDateTime>
```

Expand description

Similar to timestamp_s_to_datetime but only compute `time`
