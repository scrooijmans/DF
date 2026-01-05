# Function validate_decimal_precision_and_scale Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/types.rs.html#1351-1354" class="src">Source</a>

``` rust
pub fn validate_decimal_precision_and_scale<T>(
    precision: u8,
    scale: i8,
) -> Result<(), ArrowError>where
    T: DecimalType,
```

Expand description

Validate that `precision` and `scale` are valid for `T`

Returns an Error if:

- `precision` is zero
- `precision` is larger than `T:MAX_PRECISION`
- `scale` is larger than `T::MAX_SCALE`
- `scale` is \> `precision`
