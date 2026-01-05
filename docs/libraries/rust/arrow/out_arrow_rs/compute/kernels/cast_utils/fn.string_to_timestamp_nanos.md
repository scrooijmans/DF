# Function string_to_timestamp_nanos Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#272" class="src">Source</a>

``` rust
pub fn string_to_timestamp_nanos(s: &str) -> Result<i64, ArrowError>
```

Expand description

Accepts a string in RFC3339 / ISO8601 standard format and some variants and converts it to a nanosecond precision timestamp.

See [`string_to_datetime`](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_datetime.html "fn arrow::compute::kernels::cast_utils::string_to_datetime") for the full set of supported formats

Implements the `to_timestamp` function to convert a string to a timestamp, following the model of spark SQL’s to\_`timestamp`.

Internally, this function uses the `chrono` library for the datetime parsing

We hope to extend this function in the future with a second parameter to specifying the format string.

### <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_timestamp_nanos.html#timestamp-precision" class="doc-anchor">§</a>Timestamp Precision

Function uses the maximum precision timestamps supported by Arrow (nanoseconds stored as a 64-bit integer) timestamps. This means the range of dates that timestamps can represent is ~1677 AD to 2262 AM

### <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_timestamp_nanos.html#timezone--offset-handling" class="doc-anchor">§</a>Timezone / Offset Handling

Numerical values of timestamps are stored compared to offset UTC.

This function interprets string without an explicit time zone as timestamps relative to UTC, see [`string_to_datetime`](https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_datetime.html "fn arrow::compute::kernels::cast_utils::string_to_datetime") for alternative semantics

In particular:

``` rust
// Note all three of these timestamps are parsed as the same value
let a = string_to_timestamp_nanos("1997-01-31 09:26:56.123Z").unwrap();
let b = string_to_timestamp_nanos("1997-01-31T09:26:56.123").unwrap();
let c = string_to_timestamp_nanos("1997-01-31T14:26:56.123+05:00").unwrap();

assert_eq!(a, b);
assert_eq!(b, c);
```
