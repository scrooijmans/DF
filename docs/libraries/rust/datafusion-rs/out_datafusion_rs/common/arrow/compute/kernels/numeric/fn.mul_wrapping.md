# Function mul_wrapping Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/numeric.rs.html#59" class="src">Source</a>

``` rust
pub fn mul_wrapping(
    lhs: &dyn Datum,
    rhs: &dyn Datum,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Perform `lhs * rhs`, wrapping on overflow for [`DataType::is_integer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html#method.is_integer "method datafusion::common::arrow::datatypes::DataType::is_integer")
