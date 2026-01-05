# Function try_unary_mut Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#61-67" class="src">Source</a>

``` rust
pub fn try_unary_mut<I, F>(
    array: PrimitiveArray<I>,
    op: F,
) -> Result<Result<PrimitiveArray<I>, ArrowError>, PrimitiveArray<I>>where
    I: ArrowPrimitiveType,
    F: Fn(<I as ArrowPrimitiveType>::Native) -> Result<<I as ArrowPrimitiveType>::Native, ArrowError>,
```

Expand description

See [`PrimitiveArray::try_unary_mut`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.try_unary_mut "method arrow::array::PrimitiveArray::try_unary_mut")
