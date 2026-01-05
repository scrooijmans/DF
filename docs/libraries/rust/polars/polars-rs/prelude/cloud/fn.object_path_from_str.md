# Function object_path_from_str Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/object_store_setup.rs.html#89" class="src">Source</a>

``` rust
pub fn object_path_from_str(path: &str) -> Result<Path, PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Construct an object_store `Path` from a string without any encoding/decoding.
