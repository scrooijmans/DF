# Function get_reader_bytes Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/other.rs.html#10-12" class="src">Source</a>

``` rust
pub fn get_reader_bytes<R>(
    reader: &mut R,
) -> Result<ReaderBytes<'_>, PolarsError>where
    R: Read + MmapBytesReader + ?Sized,
```

Available on **crate feature `polars-io`** only.
