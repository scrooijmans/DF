# Function bitwise_xor Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/bitwise.rs.html#68-74" class="src">Source</a>

``` rust
pub fn bitwise_xor<T>(
    left: &PrimitiveArray<T>,
    right: &PrimitiveArray<T>,
) -> Result<PrimitiveArray<T>, ArrowError>where
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: BitXor<Output = <T as ArrowPrimitiveType>::Native>,
```

Expand description

Perform `left ^ right` operation on two arrays. If either left or right value is null then the result is also null.
