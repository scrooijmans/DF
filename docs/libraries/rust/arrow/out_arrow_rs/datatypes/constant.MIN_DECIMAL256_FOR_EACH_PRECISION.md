# Constant MIN_DECIMAL256_FOR_EACH_PRECISION Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#383" class="src">Source</a>

``` rust
pub const MIN_DECIMAL256_FOR_EACH_PRECISION: [i256; 77];
```

Expand description

`MIN_DECIMAL256_FOR_EACH_PRECISION[p]` holds the minimum [`i256`](https://docs.rs/arrow/latest/arrow/datatypes/struct.i256.html "struct arrow::datatypes::i256") value that can be stored in a [`Decimal256`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal256 "variant arrow::datatypes::DataType::Decimal256") value of precision `p`.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL256_FOR_EACH_PRECISION.html#notes" class="doc-anchor">§</a>Notes

Each element is the min value of signed 256-bit integer for the specified precision which is encoded to the 76-byte width format of little-endian.

The first element is unused and is inserted so that we can look up using precision as the index without the need to subtract 1 first.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MIN_DECIMAL256_FOR_EACH_PRECISION.html#example" class="doc-anchor">§</a>Example

``` rust
assert_eq!(MIN_DECIMAL256_FOR_EACH_PRECISION[3], i256::from(-999));
```
