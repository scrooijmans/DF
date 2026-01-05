# Function unpack_dtypes Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/datatypes/dtype.rs.html#1274" class="src">Source</a>

``` rust
pub fn unpack_dtypes(
    dtype: &DataType,
    include_compound_types: bool,
) -> HashSet<DataType, RandomState>
```
