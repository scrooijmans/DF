# Function infer_schema Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/row/mod.rs.html#101-104" class="src">Source</a>

``` rust
pub fn infer_schema(
    iter: impl Iterator<Item = Vec<(impl Into<PlSmallStr>, impl Into<DataType>)>>,
    infer_schema_length: usize,
) -> Schema<DataType>
```
