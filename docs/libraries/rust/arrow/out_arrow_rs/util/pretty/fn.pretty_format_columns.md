# Function pretty_format_columns Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/pretty.rs.html#142-145" class="src">Source</a>

``` rust
pub fn pretty_format_columns(
    col_name: &str,
    results: &[Arc<dyn Array>],
) -> Result<impl Display, ArrowError>
```

Available on **crate feature `prettyprint`** only.

Expand description

Create a visual representation of [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef")

Uses default values for display. See [`pretty_format_columns_with_options`](https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_columns_with_options.html "fn arrow::util::pretty::pretty_format_columns_with_options")

See [`pretty_format_batches`](https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches.html "fn arrow::util::pretty::pretty_format_batches") for an example
