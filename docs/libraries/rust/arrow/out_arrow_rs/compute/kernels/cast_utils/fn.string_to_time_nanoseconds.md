# Function string_to_time_nanoseconds Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#299" class="src">Source</a>

``` rust
pub fn string_to_time_nanoseconds(s: &str) -> Result<i64, ArrowError>
```

Expand description

Accepts a string in ISO8601 standard format and some variants and converts it to nanoseconds since midnight.

Examples of accepted inputs:

- `09:26:56.123 AM`
- `23:59:59`
- `6:00 pm`

Internally, this function uses the `chrono` library for the time parsing

### <a href="https://docs.rs/arrow/latest/arrow/compute/kernels/cast_utils/fn.string_to_time_nanoseconds.html#timezone--offset-handling" class="doc-anchor">§</a>Timezone / Offset Handling

This function does not support parsing strings with a timezone or offset specified, as it considers only time since midnight.
