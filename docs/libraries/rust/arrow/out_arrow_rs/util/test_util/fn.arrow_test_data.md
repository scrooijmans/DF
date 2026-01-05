# Function arrow_test_data Copy item path

<a href="https://docs.rs/arrow/latest/src/arrow/util/test_util.rs.html#78-83" class="src">Source</a>

``` rust
pub fn arrow_test_data() -> String
```

Available on **crate feature `test_utils`** only.

Expand description

Returns the arrow test data directory, which is by default stored in a git submodule rooted at `arrow/testing/data`.

The default can be overridden by the optional environment variable `ARROW_TEST_DATA`

panics when the directory can not be found.

Example:

``` rust
let testdata = arrow::util::test_util::arrow_test_data();
let csvdata = format!("{}/csv/aggregate_test_100.csv", testdata);
assert!(std::path::PathBuf::from(csvdata).exists());
```
