# Function pretty_format_batches Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/pretty.rs.html#63" class="src">Source</a>

``` rust
pub fn pretty_format_batches(
    results: &[RecordBatch],
) -> Result<impl Display, ArrowError>
```

Available on **crate feature `prettyprint`** only.

Expand description

Create a visual representation of [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es

Uses default values for display. See [`pretty_format_batches_with_options`](https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches_with_options.html "fn arrow::util::pretty::pretty_format_batches_with_options") for more control.

## <a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches.html#example" class="doc-anchor">§</a>Example

``` rust
// Note, returned object implements `Display`
let pretty_table = pretty_format_batches(&[batch]).unwrap();
let table_str = format!("Batches:\n{pretty_table}");
assert_eq!(table_str,
r#"Batches:
+---+---+
| a | b |
+---+---+
| 1 | a |
| 2 | b |
| 3 |   |
| 4 | d |
| 5 | e |
+---+---+"#);
```
