# Function expand_paths_hive Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/path_utils/mod.rs.html#204-209" class="src">Source</a>

``` rust
pub fn expand_paths_hive(
    paths: &[PlPath],
    glob: bool,
    cloud_options: &mut Option<CloudOptions>,
    check_directory_level: bool,
) -> Result<(Arc<[PlPath]>, usize), PolarsError>
```

Available on **crate feature `polars-io`** only.

Expand description

Recursively traverses directories and expands globs if `glob` is `true`. Returns the expanded paths and the index at which to start parsing hive partitions from the path.
