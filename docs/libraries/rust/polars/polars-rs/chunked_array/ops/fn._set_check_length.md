# Function \_set_check_length Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/chunkops.rs.html#106" class="src">Source</a>

``` rust
pub unsafe fn _set_check_length(check: bool)
```

Expand description

Meant for internal use. In very rare conditions this can be turned off.

## <a href="https://docs.rs/polars/latest/polars/chunked_array/ops/fn._set_check_length.html#safety" class="doc-anchor">§</a>Safety

The caller must ensure the Series that exceeds the length get’s deconstructed into array values or list values before and never is used.
