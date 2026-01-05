# Function validate_decimal32_precision Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#1024" class="src">Source</a>

``` rust
pub fn validate_decimal32_precision(
    value: i32,
    precision: u8,
) -> Result<(), ArrowError>
```

Expand description

Validates that the specified `i32` value can be properly interpreted as a [`Decimal32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") number with precision `precision`
