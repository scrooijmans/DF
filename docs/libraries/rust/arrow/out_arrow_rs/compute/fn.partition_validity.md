# Function partition_validity Copy item path

<a href="https://docs.rs/arrow-ord/56.2.0/x86_64-unknown-linux-gnu/src/arrow_ord/sort.rs.html#188" class="src">Source</a>

``` rust
pub fn partition_validity(array: &dyn Array) -> (Vec<u32>, Vec<u32>)
```

Expand description

Partition indices of an Arrow array into two categories:

- `valid`: indices of non-null elements
- `nulls`: indices of null elements

Optimized for performance with fast-path for all-valid arrays and bit-parallel scan for null-containing arrays.
