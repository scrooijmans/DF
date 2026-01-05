# Function ensure_is_constant_duration Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/windows/duration.rs.html#1031-1035" class="src">Source</a>

``` rust
pub fn ensure_is_constant_duration(
    duration: Duration,
    time_zone: Option<&TimeZone>,
    variable_name: &str,
) -> Result<(), PolarsError>
```

Available on **crate feature `temporal`** only.
