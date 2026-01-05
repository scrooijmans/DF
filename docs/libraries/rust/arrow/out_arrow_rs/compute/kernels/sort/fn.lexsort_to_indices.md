# Function lexsort_to_indices Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#905-908" class="src">Source</a>

``` rust
pub fn lexsort_to_indices(
    columns: &[SortColumn],
    limit: Option<usize>,
) -> Result<PrimitiveArray<UInt32Type>, ArrowError>
```

Expand description

Sort elements lexicographically from a list of `ArrayRef` into an unsigned integer (`UInt32Array`) of indices.

Note: for multi-column sorts without a limit, using the [row format](https://docs.rs/arrow-row/latest/arrow_row/) may be significantly faster
