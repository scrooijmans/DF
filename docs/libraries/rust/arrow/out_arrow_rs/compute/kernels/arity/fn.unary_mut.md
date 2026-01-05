# Function unary_mut Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#39-45" class="src">Source</a>

``` rust
pub fn unary_mut<I, F>(
    array: PrimitiveArray<I>,
    op: F,
) -> Result<PrimitiveArray<I>, PrimitiveArray<I>>where
    I: ArrowPrimitiveType,
    F: Fn(<I as ArrowPrimitiveType>::Native) -> <I as ArrowPrimitiveType>::Native,
```

Expand description

See [`PrimitiveArray::unary_mut`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html#method.unary_mut "method arrow::array::PrimitiveArray::unary_mut")
