# Macro assert_batches_sorted_eq Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/test_util.rs.html#121" class="src">Source</a>

``` rust
macro_rules! assert_batches_sorted_eq {
    ($EXPECTED_LINES: expr, $CHUNKS: expr) => { ... };
}
```

Expand description

Compares formatted output of a record batch with an expected vector of strings in a way that order does not matter. This is a macro so errors appear on the correct line

See [`assert_batches_eq`](https://docs.rs/datafusion/50.2.0/datafusion/macro.assert_batches_eq.html "macro datafusion::assert_batches_eq") for more details and example.

Expects to be called about like this:

`assert_batch_sorted_eq!(expected_lines: &[&str], batches: &[RecordBatch])`
