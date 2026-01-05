# Function sum_array Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#560-563" class="src">Source</a>

``` rust
pub fn sum_array<T, A>(array: A) -> Option<<T as ArrowPrimitiveType>::Native>where
    A: ArrayAccessor<Item = <T as ArrowPrimitiveType>::Native>,
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: ArrowNativeTypeOp,
```

Expand description

Returns the sum of values in the array.

This doesn’t detect overflow. Once overflowing, the result will wrap around. For an overflow-checking variant, use `sum_array_checked` instead.
