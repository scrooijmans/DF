# Function bit_or Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#736-742" class="src">Source</a>

``` rust
pub fn bit_or<T>(
    array: &PrimitiveArray<T>,
) -> Option<<T as ArrowPrimitiveType>::Native>where
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: BitOr<Output = <T as ArrowPrimitiveType>::Native> + ArrowNativeTypeOp,
```

Expand description

Returns the bitwise or of all non-null input values.

Returns `None` if the array is empty or only contains null values.
