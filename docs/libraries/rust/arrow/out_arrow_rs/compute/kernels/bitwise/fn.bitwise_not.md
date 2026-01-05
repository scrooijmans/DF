# Function bitwise_not Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/bitwise.rs.html#113-116" class="src">Source</a>

``` rust
pub fn bitwise_not<T>(
    array: &PrimitiveArray<T>,
) -> Result<PrimitiveArray<T>, ArrowError>where
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: Not<Output = <T as ArrowPrimitiveType>::Native>,
```

Expand description

Perform `!array` operation on array. If array value is null then the result is also null.
