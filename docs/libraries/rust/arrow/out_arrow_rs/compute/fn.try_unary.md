# Function try_unary Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#51-55" class="src">Source</a>

``` rust
pub fn try_unary<I, F, O>(
    array: &PrimitiveArray<I>,
    op: F,
) -> Result<PrimitiveArray<O>, ArrowError>where
    I: ArrowPrimitiveType,
    O: ArrowPrimitiveType,
    F: Fn(<I as ArrowPrimitiveType>::Native) -> Result<<O as ArrowPrimitiveType>::Native, ArrowError>,
```

Expand description

See [`PrimitiveArray::try_unary`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.try_unary "method arrow::array::PrimitiveArray::try_unary")
