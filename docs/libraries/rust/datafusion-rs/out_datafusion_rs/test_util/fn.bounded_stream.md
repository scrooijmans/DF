# Function bounded_stream Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test_util/mod.rs.html#246-255" class="src">Source</a>

``` rust
pub fn bounded_stream(
    record_batch: RecordBatch,
    limit: usize,
) -> SendableRecordBatchStream
```

Expand description

Creates a bounded stream that emits the same record batch a specified number of times. This is useful for testing purposes.
