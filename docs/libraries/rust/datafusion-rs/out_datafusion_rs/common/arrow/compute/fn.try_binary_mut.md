# Function try_binary_mut Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/arity.rs.html#305-312" class="src">Source</a>

``` rust
pub fn try_binary_mut<T, F>(
    a: PrimitiveArray<T>,
    b: &PrimitiveArray<T>,
    op: F,
) -> Result<Result<PrimitiveArray<T>, ArrowError>, PrimitiveArray<T>>where
    T: ArrowPrimitiveType,
    F: Fn(<T as ArrowPrimitiveType>::Native, <T as ArrowPrimitiveType>::Native) -> Result<<T as ArrowPrimitiveType>::Native, ArrowError>,
```

Expand description

Applies the provided fallible binary operation across `a` and `b` by mutating the mutable [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") `a` with the results.

Returns any error encountered, or collects the results into a [`PrimitiveArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.PrimitiveArray.html "struct datafusion::common::arrow::array::PrimitiveArray") as return value. If any index is null in either `a` or `b`, the corresponding index in the result will also be null.

Like [`try_unary`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.try_unary.html "fn datafusion::common::arrow::compute::try_unary") the function is only evaluated for non-null indices.

See [`binary_mut`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/compute/fn.binary_mut.html "fn datafusion::common::arrow::compute::binary_mut") for errors and buffer reuse information.
