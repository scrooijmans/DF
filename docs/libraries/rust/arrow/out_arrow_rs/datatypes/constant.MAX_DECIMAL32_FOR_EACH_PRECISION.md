# Constant MAX_DECIMAL32_FOR_EACH_PRECISION Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#994" class="src">Source</a>

``` rust
pub const MAX_DECIMAL32_FOR_EACH_PRECISION: [i32; 10];
```

Expand description

`MAX_DECIMAL32_FOR_EACH_PRECISION[p]` holds the maximum `i32` value that can be stored in [`Decimal32`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal32 "variant arrow::datatypes::DataType::Decimal32") value of precision `p`.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL32_FOR_EACH_PRECISION.html#notes" class="doc-anchor">§</a>Notes

The first element is unused and is inserted so that we can look up using precision as the index without the need to subtract 1 first.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL32_FOR_EACH_PRECISION.html#example" class="doc-anchor">§</a>Example

``` rust
assert_eq!(MAX_DECIMAL32_FOR_EACH_PRECISION[3], 999);
```
