# Function is_validate_decimal_precision Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#1124" class="src">Source</a>

``` rust
pub fn is_validate_decimal_precision(value: i128, precision: u8) -> bool
```

Expand description

Returns true if the specified `i128` value can be properly interpreted as a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") number with precision `precision`
