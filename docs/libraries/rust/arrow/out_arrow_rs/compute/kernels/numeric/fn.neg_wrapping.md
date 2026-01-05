# Function neg_wrapping Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/numeric.rs.html#179" class="src">Source</a>

``` rust
pub fn neg_wrapping(array: &dyn Array) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Negates each element of `array`, wrapping on overflow for [`DataType::is_integer`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#method.is_integer "method arrow::datatypes::DataType::is_integer")
