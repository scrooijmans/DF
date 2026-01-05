# Function arrow_test_data Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/test_util.rs.html#233" class="src">Source</a>

``` rust
pub fn arrow_test_data() -> String
```

Expand description

Returns the arrow test data directory, which is by default stored in a git submodule rooted at `testing/data`.

The default can be overridden by the optional environment variable `ARROW_TEST_DATA`

panics when the directory can not be found.

Example:

``` rust
let testdata = datafusion_common::test_util::arrow_test_data();
let csvdata = format!("{}/csv/aggregate_test_100.csv", testdata);
assert!(std::path::PathBuf::from(csvdata).exists());
```
