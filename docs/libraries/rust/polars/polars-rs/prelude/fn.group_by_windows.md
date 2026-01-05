# Function group_by_windows Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/windows/group_by.rs.html#145-154" class="src">Source</a>

``` rust
pub fn group_by_windows(
    window: Window,
    time: &[i64],
    closed_window: ClosedWindow,
    tu: TimeUnit,
    tz: &Option<TimeZone>,
    include_lower_bound: bool,
    include_upper_bound: bool,
    start_by: StartBy,
) -> Result<(Vec<[u32; 2]>, Vec<i64>, Vec<i64>), PolarsError>
```

Available on **crate feature `temporal`** only.

Expand description

Window boundaries are created based on the given `Window`, which is defined by:

- every
- period
- offset

And every window boundary we search for the values that fit that window by the given `ClosedWindow`. The groups are return as `GroupTuples` together with the lower bound and upper bound timestamps. These timestamps indicate the start (lower) and end (upper) of the window of that group.

If `include_boundaries` is `false` those `lower` and `upper` vectors will be empty.
