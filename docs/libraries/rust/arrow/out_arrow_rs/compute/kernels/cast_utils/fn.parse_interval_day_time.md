# Function parse_interval_day_time Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#1006-1008" class="src">Source</a>

``` rust
pub fn parse_interval_day_time(
    value: &str,
) -> Result<<IntervalDayTimeType as ArrowPrimitiveType>::Native, ArrowError>
```

Expand description

Parse human-readable interval string to Arrow [IntervalDayTimeType](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalDayTimeType.html "struct arrow::datatypes::IntervalDayTimeType")
