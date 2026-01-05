# Function sum Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#819-821" class="src">Source</a>

``` rust
pub fn sum<T>(
    array: &PrimitiveArray<T>,
) -> Option<<T as ArrowPrimitiveType>::Native>where
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: ArrowNativeTypeOp,
```

Expand description

Returns the sum of values in the primitive array.

Returns `None` if the array is empty or only contains null values.

This doesn’t detect overflow in release mode by default. Once overflowing, the result will wrap around. For an overflow-checking variant, use `sum_checked` instead.
