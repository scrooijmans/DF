# Function parquet_test_data Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/test_util.rs.html#256" class="src">Source</a>

``` rust
pub fn parquet_test_data() -> String
```

Available on **crate feature `parquet`** only.

Expand description

Returns the parquet test data directory, which is by default stored in a git submodule rooted at `parquet-testing/data`.

The default can be overridden by the optional environment variable `PARQUET_TEST_DATA`

panics when the directory can not be found.

Example:

``` rust
let testdata = datafusion_common::test_util::parquet_test_data();
let filename = format!("{}/binary.parquet", testdata);
assert!(std::path::PathBuf::from(filename).exists());
```
