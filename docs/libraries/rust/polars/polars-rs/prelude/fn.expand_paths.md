# Function expand_paths Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/path_utils/mod.rs.html#163-167" class="src">Source</a>

``` rust
pub fn expand_paths(
    paths: &[PlPath],
    glob: bool,
    cloud_options: &mut Option<CloudOptions>,
) -> Result<Arc<[PlPath]>, PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Recursively traverses directories and expands globs if `glob` is `true`.
