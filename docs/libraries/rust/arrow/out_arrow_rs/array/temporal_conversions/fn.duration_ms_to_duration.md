# Function duration_ms_to_duration Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#234" class="src">Source</a>

``` rust
pub fn duration_ms_to_duration(v: i64) -> TimeDelta
```

👎Deprecated since 55.2.0: Use `try_duration_ms_to_duration` instead

Expand description

converts a `i64` representing a `duration(ms)` to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")
