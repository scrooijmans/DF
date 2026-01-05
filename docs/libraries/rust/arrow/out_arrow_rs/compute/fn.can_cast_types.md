# Function can_cast_types Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/cast/mod.rs.html#92" class="src">Source</a>

``` rust
pub fn can_cast_types(from_type: &DataType, to_type: &DataType) -> bool
```

Expand description

Return true if a value of type `from_type` can be cast into a value of `to_type`.

See [`cast_with_options`](https://docs.rs/arrow/latest/arrow/compute/fn.cast_with_options.html "fn arrow::compute::cast_with_options") for more information
