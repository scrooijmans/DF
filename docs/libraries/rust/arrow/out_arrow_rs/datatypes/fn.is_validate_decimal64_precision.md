# Function is_validate_decimal64_precision Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#1087" class="src">Source</a>

``` rust
pub fn is_validate_decimal64_precision(value: i64, precision: u8) -> bool
```

Expand description

Returns true if the specified `i64` value can be properly interpreted as a [`Decimal64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") number with precision `precision`
