# Function rem Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/numeric.rs.html#77" class="src">Source</a>

``` rust
pub fn rem(
    lhs: &dyn Datum,
    rhs: &dyn Datum,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Perform `lhs % rhs`

Division by zero will result in an error, with exception to floating point numbers, which instead follow the IEEE 754 rules

`signed_integer::MIN % -1` will not result in an error but return 0
