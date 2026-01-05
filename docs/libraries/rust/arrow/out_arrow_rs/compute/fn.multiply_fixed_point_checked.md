# Function multiply_fixed_point_checked Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/arithmetic.rs.html#101-105" class="src">Source</a>

``` rust
pub fn multiply_fixed_point_checked(
    left: &PrimitiveArray<Decimal128Type>,
    right: &PrimitiveArray<Decimal128Type>,
    required_scale: i8,
) -> Result<PrimitiveArray<Decimal128Type>, ArrowError>
```

Expand description

Perform `left * right` operation on two decimal arrays. If either left or right value is null then the result is also null.

This performs decimal multiplication which allows precision loss if an exact representation is not possible for the result, according to the required scale. In the case, the result will be rounded to the required scale.

If the required scale is greater than the product scale, an error is returned.

It is implemented for compatibility with precision loss `multiply` function provided by other data processing engines. For multiplication with precision loss detection, use `multiply` or `multiply_checked` instead.
