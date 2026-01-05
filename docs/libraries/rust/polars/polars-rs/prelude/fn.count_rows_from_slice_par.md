# Function count_rows_from_slice_par Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/parser.rs.html#67-77" class="src">Source</a>

``` rust
pub fn count_rows_from_slice_par(
    bytes: &[u8],
    separator: u8,
    quote_char: Option<u8>,
    comment_prefix: Option<&CommentPrefix>,
    eol_char: u8,
    has_header: bool,
    skip_lines: usize,
    skip_rows_before_header: usize,
    skip_rows_after_header: usize,
) -> Result<usize, PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Read the number of rows without parsing columns useful for count(\*) queries
