# Function extract_row_at_idx_to_buf Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/utils/mod.rs.html#86-90" class="src">Source</a>

``` rust
pub fn extract_row_at_idx_to_buf(
    columns: &[Arc<dyn Array>],
    idx: usize,
    buf: &mut Vec<ScalarValue>,
) -> Result<(), DataFusionError>
```

Expand description

Extracts a row at the specified index from a set of columns and stores it in the provided buffer.
