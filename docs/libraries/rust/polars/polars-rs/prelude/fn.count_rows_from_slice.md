# Function count_rows_from_slice Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/parser.rs.html#150-156" class="src">Source</a>

``` rust
pub fn count_rows_from_slice(
    bytes: &[u8],
    quote_char: Option<u8>,
    comment_prefix: Option<&CommentPrefix>,
    eol_char: u8,
    has_header: bool,
) -> Result<usize, PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Read the number of rows without parsing columns
