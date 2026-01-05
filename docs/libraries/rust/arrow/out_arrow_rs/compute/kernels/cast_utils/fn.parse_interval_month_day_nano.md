# Function parse_interval_month_day_nano Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#1032-1034" class="src">Source</a>

``` rust
pub fn parse_interval_month_day_nano(
    value: &str,
) -> Result<<IntervalMonthDayNanoType as ArrowPrimitiveType>::Native, ArrowError>
```

Expand description

Parse human-readable interval string to Arrow [IntervalMonthDayNanoType](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalMonthDayNanoType.html "struct arrow::datatypes::IntervalMonthDayNanoType")
