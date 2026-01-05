# Function pretty_format_batches_with_options Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/pretty.rs.html#130-133" class="src">Source</a>

``` rust
pub fn pretty_format_batches_with_options(
    results: &[RecordBatch],
    options: &FormatOptions<'_>,
) -> Result<impl Display, ArrowError>
```

Available on **crate feature `prettyprint`** only.

Expand description

Create a visual representation of [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es with formatting options.

## <a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches_with_options.html#arguments" class="doc-anchor">§</a>Arguments

- `results` - A slice of record batches to display
- `options` - [`FormatOptions`](https://docs.rs/arrow/latest/arrow/util/display/struct.FormatOptions.html "struct arrow::util::display::FormatOptions") that control the resulting display

## <a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches_with_options.html#example" class="doc-anchor">§</a>Example

``` rust
let options = FormatOptions::new()
  .with_null("<NULL>");
// Note, returned object implements `Display`
let pretty_table = pretty_format_batches_with_options(&[batch], &options).unwrap();
let table_str = format!("Batches:\n{pretty_table}");
assert_eq!(table_str,
r#"Batches:
+---+--------+
| a | b      |
+---+--------+
| 1 | a      |
| 2 | <NULL> |
+---+--------+"#);
```
