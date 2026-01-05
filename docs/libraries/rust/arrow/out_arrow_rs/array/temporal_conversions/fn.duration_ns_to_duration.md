# Function duration_ns_to_duration Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#252" class="src">Source</a>

``` rust
pub fn duration_ns_to_duration(v: i64) -> TimeDelta
```

Expand description

converts a `i64` representing a `duration(ns)` to [`Duration`](https://docs.rs/chrono/0.4.42/x86_64-unknown-linux-gnu/chrono/type.Duration.html "type chrono::Duration")
