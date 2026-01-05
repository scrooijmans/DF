# Function try_binary Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#254-261" class="src">Source</a>

``` rust
pub fn try_binary<A, B, F, O>(
    a: A,
    b: B,
    op: F,
) -> Result<PrimitiveArray<O>, ArrowError>where
    A: ArrayAccessor,
    B: ArrayAccessor,
    O: ArrowPrimitiveType,
    F: Fn(<A as ArrayAccessor>::Item, <B as ArrayAccessor>::Item) -> Result<<O as ArrowPrimitiveType>::Native, ArrowError>,
```

Expand description

Applies the provided fallible binary operation across `a` and `b`.

This will return any error encountered, or collect the results into a [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray"). If any index is null in either `a` or `b`, the corresponding index in the result will also be null

Like [`try_unary`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.try_unary.html "fn datafusion::common::arrow::compute::try_unary") the function is only evaluated for non-null indices

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/kernels/arity/fn.try_binary.html#error" class="doc-anchor">§</a>Error

Return an error if the arrays have different lengths or the operation is under erroneous
