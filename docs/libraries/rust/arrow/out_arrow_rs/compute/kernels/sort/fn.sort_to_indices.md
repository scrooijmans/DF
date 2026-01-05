# Function sort_to_indices Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#269-273" class="src">Source</a>

``` rust
pub fn sort_to_indices(
    array: &dyn Array,
    options: Option<SortOptions>,
    limit: Option<usize>,
) -> Result<PrimitiveArray<UInt32Type>, ArrowError>
```

Expand description

Sort elements from `ArrayRef` into an unsigned integer (`UInt32Array`) of indices. Floats are sorted using IEEE 754 totalOrder. `limit` is an option for [partial_sort](https://docs.rs/arrow/latest/arrow/compute/fn.partial_sort.html "fn arrow::compute::partial_sort").
