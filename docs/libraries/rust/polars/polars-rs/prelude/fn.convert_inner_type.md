# Function convert_inner_type Copy item path

<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/src/polars_arrow/legacy/array/mod.rs.html#200" class="src">Source</a>

``` rust
pub fn convert_inner_type(
    array: &(dyn Array + 'static),
    dtype: &ArrowDataType,
) -> Box<dyn Array>
```

Expand description

Cast null arrays to inner type and ensure that all offsets remain correct
