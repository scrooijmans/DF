# Function base_utc_offset Copy item path

<a href="https://docs.rs/polars-time/0.51.0/x86_64-unknown-linux-gnu/src/polars_time/base_utc_offset.rs.html#14-18" class="src">Source</a>

``` rust
pub fn base_utc_offset(
    ca: &Logical<DatetimeType, Int64Type>,
    time_unit: &TimeUnit,
    time_zone: &Tz,
) -> Logical<DurationType, Int64Type>
```

Available on **crate feature `temporal`** only.
