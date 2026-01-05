# Constant MIN_DECIMAL128_FOR_EACH_PRECISION Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#863" class="src">Source</a>

``` rust
pub const MIN_DECIMAL128_FOR_EACH_PRECISION: [i128; 39];
```

Expand description

`MIN_DECIMAL_FOR_EACH_PRECISION[p]` holds the minimum `i128` value that can be stored in a [`Decimal128`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal128 "variant arrow::datatypes::DataType::Decimal128") value of precision `p`.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL128_FOR_EACH_PRECISION.html#notes" class="doc-anchor">§</a>Notes

The first element is unused and is inserted so that we can look up using precision as the index without the need to subtract 1 first.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL128_FOR_EACH_PRECISION.html#example" class="doc-anchor">§</a>Example

``` rust
assert_eq!(MIN_DECIMAL128_FOR_EACH_PRECISION[3], -999);
```
