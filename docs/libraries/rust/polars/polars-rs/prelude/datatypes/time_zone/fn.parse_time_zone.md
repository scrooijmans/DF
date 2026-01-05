# Function parse_time_zone Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/temporal/time_zone.rs.html#158" class="src">Source</a>

``` rust
pub fn parse_time_zone(tz: &str) -> Result<Tz, PolarsError>
```

Expand description

Parse a time zone string to [`chrono_tz::Tz`](https://docs.rs/chrono-tz/0.10.3/x86_64-unknown-linux-gnu/chrono_tz/timezones/enum.Tz.html "enum chrono_tz::timezones::Tz")
