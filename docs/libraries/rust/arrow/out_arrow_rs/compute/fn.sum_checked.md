# Function sum_checked Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#771-774" class="src">Source</a>

``` rust
pub fn sum_checked<T>(
    array: &PrimitiveArray<T>,
) -> Result<Option<<T as ArrowPrimitiveType>::Native>, ArrowError>where
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: ArrowNativeTypeOp,
```

Expand description

Returns the sum of values in the primitive array.

Returns `Ok(None)` if the array is empty or only contains null values.

This detects overflow and returns an `Err` for that. For an non-overflow-checking variant, use `sum` instead.
