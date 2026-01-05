# Function maybe_decompress_bytes Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/compression.rs.html#38" class="src">Source</a>

``` rust
pub fn maybe_decompress_bytes<'a>(
    bytes: &'a [u8],
    out: &'a mut Vec<u8>,
) -> Result<&'a [u8], PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Decompress `bytes` if compression is detected, otherwise simply return it. An `out` vec must be given for ownership of the decompressed data.
