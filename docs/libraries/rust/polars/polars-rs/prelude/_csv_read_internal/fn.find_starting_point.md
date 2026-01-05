# Function find_starting_point Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/read_impl.rs.html#596-606" class="src">Source</a>

``` rust
pub fn find_starting_point(
    bytes: &[u8],
    quote_char: Option<u8>,
    eol_char: u8,
    schema_len: usize,
    skip_lines: usize,
    skip_rows_before_header: usize,
    skip_rows_after_header: usize,
    comment_prefix: Option<&CommentPrefix>,
    has_header: bool,
) -> Result<usize, PolarsError>
```

Available on **crate feature `polars-io`** only.
