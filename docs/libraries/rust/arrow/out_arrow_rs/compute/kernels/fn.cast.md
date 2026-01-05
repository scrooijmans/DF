# Function cast Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/cast/mod.rs.html#339" class="src">Source</a>

``` rust
pub fn cast(
    array: &dyn Array,
    to_type: &DataType,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Cast `array` to the provided data type and return a new Array with type `to_type`, if possible.

See [`cast_with_options`](https://docs.rs/arrow/latest/arrow/compute/fn.cast_with_options.html "fn arrow::compute::cast_with_options") for more information
