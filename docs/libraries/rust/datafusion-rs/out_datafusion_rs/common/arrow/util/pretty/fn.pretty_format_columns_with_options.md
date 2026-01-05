# Function pretty_format_columns_with_options Copy item path

<a href="https://docs.rs/arrow-cast/56.0.0/x86_64-unknown-linux-gnu/src/arrow_cast/pretty.rs.html#153-157" class="src">Source</a>

``` rust
pub fn pretty_format_columns_with_options(
    col_name: &str,
    results: &[Arc<dyn Array>],
    options: &FormatOptions<'_>,
) -> Result<impl Display, ArrowError>
```

Expand description

Create a visual representation of [`ArrayRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.ArrayRef.html "type datafusion::common::arrow::array::ArrayRef") with formatting options.

See [`pretty_format_batches_with_options`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/util/pretty/fn.pretty_format_batches_with_options.html "fn datafusion::common::arrow::util::pretty::pretty_format_batches_with_options") for an example
