# Function compare_byte_view Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/cmp.rs.html#641-646" class="src">Source</a>

``` rust
pub fn compare_byte_view<T>(
    left: &GenericByteViewArray<T>,
    left_idx: usize,
    right: &GenericByteViewArray<T>,
    right_idx: usize,
) -> Orderingwhere
    T: ByteViewType,
```

Expand description

Compares two [`GenericByteViewArray`](https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html "struct arrow::array::GenericByteViewArray") at index `left_idx` and `right_idx`
