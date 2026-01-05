# Function array_value_to_string Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/display.rs.html#1083" class="src">Source</a>

``` rust
pub fn array_value_to_string(
    column: &dyn Array,
    row: usize,
) -> Result<String, ArrowError>
```

Expand description

Get the value at the given row in an array as a String.

Note this function is quite inefficient and is unlikely to be suitable for converting large arrays or record batches.

Please see [`ArrayFormatter`](https://docs.rs/arrow/latest/arrow/util/display/struct.ArrayFormatter.html "struct arrow::util::display::ArrayFormatter") for a more performant interface
