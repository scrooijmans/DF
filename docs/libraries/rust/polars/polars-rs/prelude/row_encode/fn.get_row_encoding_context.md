# Function get_row_encoding_context Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/chunked_array/ops/row_encode.rs.html#75" class="src">Source</a>

``` rust
pub fn get_row_encoding_context(dtype: &DataType) -> Option<RowEncodingContext>
```

Expand description

Get the [`RowEncodingContext`](https://docs.rs/polars-row/0.51.0/x86_64-unknown-linux-gnu/polars_row/row/enum.RowEncodingContext.html "enum polars_row::row::RowEncodingContext") for a certain [`DataType`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html "enum polars::prelude::DataType").

This should be given the logical type in order to communicate Polars datatype information down into the row encoding / decoding.
