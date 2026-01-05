# Function parse_interval_year_month Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#990-992" class="src">Source</a>

``` rust
pub fn parse_interval_year_month(
    value: &str,
) -> Result<<IntervalYearMonthType as ArrowPrimitiveType>::Native, ArrowError>
```

Expand description

Parse human-readable interval string to Arrow [IntervalYearMonthType](https://docs.rs/arrow/latest/arrow/datatypes/struct.IntervalYearMonthType.html "struct arrow::datatypes::IntervalYearMonthType")
