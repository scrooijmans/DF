# Module pretty Copy item path

<a href="https://docs.rs/arrow-cast/56.2.0/x86_64-unknown-linux-gnu/src/arrow_cast/lib.rs.html#31" class="src">Source</a>

Available on **crate feature `prettyprint`** only.

Expand description

Utilities for pretty printing [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es and [`Array`](https://docs.rs/arrow/latest/arrow/array/trait.Array.html "trait arrow::array::Array")s.

Note this module is not available unless `feature = "prettyprint"` is enabled.

## Functions<a href="https://docs.rs/arrow/latest/arrow/util/pretty/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches.html" class="fn" title="fn arrow::util::pretty::pretty_format_batches">pretty_format_batches</a>  
Create a visual representation of [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es

<a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches_with_options.html" class="fn" title="fn arrow::util::pretty::pretty_format_batches_with_options">pretty_format_batches_with_options</a>  
Create a visual representation of [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es with formatting options.

<a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_batches_with_schema.html" class="fn" title="fn arrow::util::pretty::pretty_format_batches_with_schema">pretty_format_batches_with_schema</a>  
Create a visual representation of [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch")es with a provided schema.

<a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_columns.html" class="fn" title="fn arrow::util::pretty::pretty_format_columns">pretty_format_columns</a>  
Create a visual representation of [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef")

<a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.pretty_format_columns_with_options.html" class="fn" title="fn arrow::util::pretty::pretty_format_columns_with_options">pretty_format_columns_with_options</a>  
Create a visual representation of [`ArrayRef`](https://docs.rs/arrow/latest/arrow/array/type.ArrayRef.html "type arrow::array::ArrayRef") with formatting options.

<a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.print_batches.html" class="fn" title="fn arrow::util::pretty::print_batches">print_batches</a>  
Prints a visual representation of record batches to stdout

<a href="https://docs.rs/arrow/latest/arrow/util/pretty/fn.print_columns.html" class="fn" title="fn arrow::util::pretty::print_columns">print_columns</a>  
Prints a visual representation of a list of column to stdout
