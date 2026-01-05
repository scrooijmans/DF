# Function validate_decimal256_precision Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#1135" class="src">Source</a>

``` rust
pub fn validate_decimal256_precision(
    value: i256,
    precision: u8,
) -> Result<(), ArrowError>
```

Expand description

Validates that the specified `i256` of value can be properly interpreted as a [`Decimal256`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") number with precision `precision`
