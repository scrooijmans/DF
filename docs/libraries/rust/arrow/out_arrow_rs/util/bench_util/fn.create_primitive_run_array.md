# Function create_primitive_run_array Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/bench_util.rs.html#381-410" class="src">Source</a>

``` rust
pub fn create_primitive_run_array<R: RunEndIndexType, V: ArrowPrimitiveType>(
    logical_array_len: usize,
    physical_array_len: usize,
) -> RunArray<R>
```

Available on **crate feature `test_utils`** only.

Expand description

Create primitive run array for given logical and physical array lengths
