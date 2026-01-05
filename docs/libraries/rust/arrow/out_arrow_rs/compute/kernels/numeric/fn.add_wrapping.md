# Function add_wrapping Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/numeric.rs.html#39" class="src">Source</a>

``` rust
pub fn add_wrapping(
    lhs: &dyn Datum,
    rhs: &dyn Datum,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Perform `lhs + rhs`, wrapping on overflow for [`DataType::is_integer`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#method.is_integer "method arrow::datatypes::DataType::is_integer")
