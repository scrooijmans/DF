# Macro assert_batches_eq Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/test_util.rs.html#72" class="src">Source</a>

``` rust
macro_rules! assert_batches_eq {
    ($EXPECTED_LINES: expr, $CHUNKS: expr) => { ... };
}
```

Expand description

Compares formatted output of a record batch with an expected vector of strings, with the result of pretty formatting record batches. This is a macro so errors appear on the correct line

Designed so that failure output can be directly copy/pasted into the test code as expected results.

Expects to be called about like this:

`assert_batches_eq!(expected_lines: &[&str], batches: &[RecordBatch])`

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/macro.assert_batches_eq.html#example" class="doc-anchor">§</a>Example

``` rust
let col: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
 let batch = RecordBatch::try_from_iter([("column", col)]).unwrap();
// Expected output is a vec of strings
let expected = vec![
    "+--------+",
    "| column |",
    "+--------+",
    "| 1      |",
    "| 2      |",
    "+--------+",
];
// compare the formatted output of the record batch with the expected output
assert_batches_eq!(expected, &[batch]);
```
