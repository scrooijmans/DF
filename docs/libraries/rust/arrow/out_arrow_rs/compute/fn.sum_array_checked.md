# Function sum_array_checked Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#594-599" class="src">Source</a>

``` rust
pub fn sum_array_checked<T, A>(
    array: A,
) -> Result<Option<<T as ArrowPrimitiveType>::Native>, ArrowError>where
    A: ArrayAccessor<Item = <T as ArrowPrimitiveType>::Native>,
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: ArrowNativeTypeOp,
```

Expand description

Returns the sum of values in the array.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant, use `sum_array` instead.
