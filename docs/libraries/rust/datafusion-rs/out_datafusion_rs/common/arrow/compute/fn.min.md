# Function min Copy item path

<a href="https://docs.rs/arrow-arith/56.0.0/x86_64-unknown-linux-gnu/src/arrow_arith/aggregate.rs.html#828-830" class="src">Source</a>

``` rust
pub fn min<T>(
    array: &PrimitiveArray<T>,
) -> Option<<T as ArrowPrimitiveType>::Native>where
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: PartialOrd,
```

Expand description

Returns the minimum value in the array, according to the natural order. For floating point arrays any NaN values are considered to be greater than any other non-null value
