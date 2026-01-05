# Function filter_record_batch Copy item path

<a href="https://docs.rs/arrow-select/56.2.0/x86_64-unknown-linux-gnu/src/arrow_select/filter.rs.html#171-174" class="src">Source</a>

``` rust
pub fn filter_record_batch(
    record_batch: &RecordBatch,
    predicate: &BooleanArray,
) -> Result<RecordBatch, ArrowError>
```

Expand description

Returns a filtered [RecordBatch](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch") where the corresponding elements of `predicate` are true.

This is the equivalent of calling [filter](https://docs.rs/arrow/latest/arrow/compute/fn.filter.html "fn arrow::compute::filter") on each column of the [RecordBatch](https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html "struct arrow::array::RecordBatch").
