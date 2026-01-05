# Function group_by_values Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/windows/group_by.rs.html#607-614" class="src">Source</a>

``` rust
pub fn group_by_values(
    period: Duration,
    offset: Duration,
    time: &[i64],
    closed_window: ClosedWindow,
    tu: TimeUnit,
    tz: Option<Tz>,
) -> Result<Vec<[u32; 2]>, PolarsError>
```

Available on **crate feature `temporal`** only.

Expand description

Different from `group_by_windows`, where define window buckets and search which values fit that pre-defined bucket.

This function defines every window based on the: - timestamp (lower bound) - timestamp + period (upper bound) where timestamps are the individual values in the array `time`
