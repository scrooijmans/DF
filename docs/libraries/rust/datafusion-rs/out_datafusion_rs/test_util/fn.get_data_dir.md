# Function get_data_dir Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/test_util.rs.html#279-282" class="src">Source</a>

``` rust
pub fn get_data_dir(
    udf_env: &str,
    submodule_data: &str,
) -> Result<PathBuf, Box<dyn Error>>
```

Expand description

Returns a directory path for finding test data.

udf_env: name of an environment variable

submodule_dir: fallback path (relative to CARGO_MANIFEST_DIR)

Returns either: The path referred to in `udf_env` if that variable is set and refers to a directory The submodule_data directory relative to CARGO_MANIFEST_PATH
