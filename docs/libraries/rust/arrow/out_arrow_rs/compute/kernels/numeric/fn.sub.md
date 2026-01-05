# Function sub Copy item path

<a href="https://docs.rs/arrow-arith/56.2.0/x86_64-unknown-linux-gnu/src/arrow_arith/numeric.rs.html#44" class="src">Source</a>

``` rust
pub fn sub(
    lhs: &dyn Datum,
    rhs: &dyn Datum,
) -> Result<Arc<dyn Array>, ArrowError>
```

Expand description

Perform `lhs - rhs`, returning an error on overflow
