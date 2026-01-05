# Function in_list Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/comparison.rs.html#33-39" class="src">Source</a>

``` rust
pub fn in_list<T, OffsetSize>(
    left: &PrimitiveArray<T>,
    right: &GenericListArray<OffsetSize>,
) -> Result<BooleanArray, ArrowError>where
    T: ArrowNumericType,
    OffsetSize: OffsetSizeTrait,
```

Expand description

Checks if a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") contains a value in the [`PrimitiveArray`](https://docs.rs/arrow/latest/arrow/array/struct.PrimitiveArray.html "struct arrow::array::PrimitiveArray")
