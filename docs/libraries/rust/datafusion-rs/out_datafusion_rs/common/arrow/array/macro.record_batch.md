# Macro record_batch Copy item path

<a href="https://docs.rs/arrow-array/56.0.0/x86_64-unknown-linux-gnu/src/arrow_array/record_batch.rs.html#157" class="src">Source</a>

``` rust
macro_rules! record_batch {
    ($(($name: expr, $type: ident, [$($values: expr),*])),*) => { ... };
}
```

Expand description

Creates a record batch from literal slice of values, suitable for rapid testing and development.

Example:

``` rust
use arrow_array::record_batch;
use arrow_schema;

let batch = record_batch!(
    ("a", Int32, [1, 2, 3]),
    ("b", Float64, [Some(4.0), None, Some(5.0)]),
    ("c", Utf8, ["alpha", "beta", "gamma"])
);
```

Due to limitation of [`create_array!`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/macro.create_array.html "macro datafusion::common::arrow::array::create_array") macro, support for limited data types is available.
