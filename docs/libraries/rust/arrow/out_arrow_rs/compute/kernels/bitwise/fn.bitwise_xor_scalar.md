# Function bitwise_xor_scalar Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/bitwise.rs.html#163-169" class="src">Source</a>

``` rust
pub fn bitwise_xor_scalar<T>(
    array: &PrimitiveArray<T>,
    scalar: <T as ArrowPrimitiveType>::Native,
) -> Result<PrimitiveArray<T>, ArrowError>where
    T: ArrowNumericType,
    <T as ArrowPrimitiveType>::Native: BitXor<Output = <T as ArrowPrimitiveType>::Native>,
```

Expand description

Perform bitwise `xor` every value in an array with the scalar. If any value in the array is null then the result is also null.
