# Function in_list_utf8 Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/comparison.rs.html#74-79" class="src">Source</a>

``` rust
pub fn in_list_utf8<OffsetSize>(
    left: &GenericByteArray<GenericStringType<OffsetSize>>,
    right: &GenericListArray<i32>,
) -> Result<BooleanArray, ArrowError>where
    OffsetSize: OffsetSizeTrait,
```

Expand description

Checks if a [`GenericListArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericListArray.html "struct arrow::array::GenericListArray") contains a value in the [`GenericStringArray`](https://docs.rs/arrow/latest/arrow/array/type.GenericStringArray.html "type arrow::array::GenericStringArray")
