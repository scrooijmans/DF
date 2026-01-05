# Function parse_decimal Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/parse.rs.html#854-858" class="src">Source</a>

``` rust
pub fn parse_decimal<T>(
    s: &str,
    precision: u8,
    scale: i8,
) -> Result<<T as ArrowPrimitiveType>::Native, ArrowError>where
    T: DecimalType,
```

Expand description

Parse the string format decimal value to i128/i256 format and checking the precision and scale. The result value can’t be out of bounds.
