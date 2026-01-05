# Function columns_to_projection Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/other.rs.html#67-70" class="src">Source</a>

``` rust
pub fn columns_to_projection<T>(
    columns: &[T],
    schema: &Schema<Field>,
) -> Result<Vec<usize>, PolarsError>where
    T: AsRef<str>,
```

Available on **crate feature `polars-io`** only.
