# Function chunk_df_for_writing Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/chunks.rs.html#88-91" class="src">Source</a>

``` rust
pub fn chunk_df_for_writing(
    df: &mut DataFrame,
    row_group_size: usize,
) -> Result<Cow<'_, DataFrame>, PolarsError>
```

Expand description

Split DataFrame into chunks in preparation for writing. The chunks have a maximum number of rows per chunk to ensure reasonable memory efficiency when reading the resulting file, and a minimum size per chunk to ensure reasonable performance when writing.
