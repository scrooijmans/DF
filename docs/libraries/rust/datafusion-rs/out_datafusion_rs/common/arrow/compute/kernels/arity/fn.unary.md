# Function unary Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#29-33" class="src">Source</a>

``` rust
pub fn unary<I, F, O>(array: &PrimitiveArray<I>, op: F) -> PrimitiveArray<O>where
    I: ArrowPrimitiveType,
    O: ArrowPrimitiveType,
    F: Fn(<I as ArrowPrimitiveType>::Native) -> <O as ArrowPrimitiveType>::Native,
```

Expand description

See [`PrimitiveArray::unary`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html#method.unary "method datafusion::common::arrow::array::PrimitiveArray::unary")
