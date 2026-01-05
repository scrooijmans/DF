# Function check_record_batches Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/test_utils.rs.html#34-78" class="src">Source</a>

``` rust
pub fn check_record_batches(
    record_batches: Vec<RecordBatch>,
    expected_schema: Expect,
    expected_data: Expect,
    ignore_check_columns: &[&str],
    sort_column: Option<&str>,
)
```

Expand description

Snapshot testing to check the resulting record batch.

- `expected_schema/data`: put `expect![[""]]` as a placeholder, and then run test with `UPDATE_EXPECT=1 cargo test` to automatically update the result, or use rust-analyzer (see [video](https://github.com/rust-analyzer/expect-test)). Check the doc of [`expect_test`](https://docs.rs/expect-test/1.5.1/x86_64-unknown-linux-gnu/expect_test/index.html "mod expect_test") for more details.
- `ignore_check_columns`: Some columns are not stable, so we can skip them.
- `sort_column`: The order of the data might be non-deterministic, so we can sort it by a column.
