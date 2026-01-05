# Constant MAX_DECIMAL_FOR_EACH_PRECISION Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#700" class="src">Source</a>

``` rust
pub const MAX_DECIMAL_FOR_EACH_PRECISION: [i128; 38];
```

👎Deprecated since 54.1.0: Use MAX_DECIMAL128_FOR_EACH_PRECISION (note indexes are different)

Expand description

`MAX_DECIMAL_FOR_EACH_PRECISION[p-1]` holds the maximum `i128` value that can be stored in a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") value of precision `p`
