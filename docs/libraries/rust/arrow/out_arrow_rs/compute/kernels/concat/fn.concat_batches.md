# Function concat_batches Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/concat.rs.html#470-473" class="src">Source</a>

``` rust
pub fn concat_batches<'a>(
    schema: &Arc<Schema>,
    input_batches: impl IntoIterator<Item = &'a RecordBatch>,
) -> Result<RecordBatch, ArrowError>
```

Expand description

Concatenates `batches` together into a single [`RecordBatch`](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").

The output batch has the specified `schemas`; The schema of the input are ignored.

Returns an error if the types of underlying arrays are different.
