# Macro record_batch Copy item path

<a href="https://docs.rs/datafusion-common/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_common/test_util.rs.html#377" class="src">Source</a>

``` rust
macro_rules! record_batch {
    ($(($name: expr, $type: ident, $values: expr)),*) => { ... };
}
```

Expand description

Creates a record batch from literal slice of values, suitable for rapid testing and development.

Example:

``` rust
use datafusion_common::record_batch;
let batch = record_batch!(
    ("a", Int32, vec![1, 2, 3]),
    ("b", Float64, vec![Some(4.0), None, Some(5.0)]),
    ("c", Utf8, vec!["alpha", "beta", "gamma"])
);
```
