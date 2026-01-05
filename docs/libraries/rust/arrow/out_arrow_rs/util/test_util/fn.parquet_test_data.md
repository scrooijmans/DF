# Function parquet_test_data Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/test_util.rs.html#100-105" class="src">Source</a>

``` rust
pub fn parquet_test_data() -> String
```

Available on **crate feature `test_utils`** only.

Expand description

Returns the parquest test data directory, which is by default stored in a git submodule rooted at `arrow/parquest-testing/data`.

The default can be overridden by the optional environment variable `PARQUET_TEST_DATA`

panics when the directory can not be found.

Example:

``` rust
let testdata = arrow::util::test_util::parquet_test_data();
let filename = format!("{}/binary.parquet", testdata);
assert!(std::path::PathBuf::from(filename).exists());
```
