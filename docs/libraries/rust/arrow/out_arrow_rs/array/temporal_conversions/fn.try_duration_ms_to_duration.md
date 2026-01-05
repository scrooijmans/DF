# Function try_duration_ms_to_duration Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/temporal_conversions.rs.html#240" class="src">Source</a>

``` rust
pub fn try_duration_ms_to_duration(v: i64) -> Option<TimeDelta>
```

Expand description

converts a `i64` representing a `duration(ms)` to [`Option<Duration>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")
