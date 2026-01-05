# Constant MAX_DECIMAL64_FOR_EACH_PRECISION Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/decimal.rs.html#920" class="src">Source</a>

``` rust
pub const MAX_DECIMAL64_FOR_EACH_PRECISION: [i64; 19];
```

Expand description

`MAX_DECIMAL64_FOR_EACH_PRECISION[p]` holds the maximum `i64` value that can be stored in [`Decimal64`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html#variant.Decimal64 "variant arrow::datatypes::DataType::Decimal64") value of precision `p`.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL64_FOR_EACH_PRECISION.html#notes" class="doc-anchor">§</a>Notes

The first element is unused and is inserted so that we can look up using precision as the index without the need to subtract 1 first.

## <a href="https://docs.rs/arrow/latest/arrow/datatypes/constant.MAX_DECIMAL64_FOR_EACH_PRECISION.html#example" class="doc-anchor">§</a>Example

``` rust
assert_eq!(MAX_DECIMAL64_FOR_EACH_PRECISION[3], 999);
```
