# Function div Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/numeric.rs.html#67" class="src">Source</a>

``` rust
pub fn div(
    lhs: &dyn Datum,
    rhs: &dyn Datum,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Perform `lhs / rhs`

Overflow or division by zero will result in an error, with exception to floating point numbers, which instead follow the IEEE 754 rules
