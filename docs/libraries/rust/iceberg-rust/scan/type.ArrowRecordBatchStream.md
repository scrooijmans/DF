# Type Alias ArrowRecordBatchStream Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/scan/mod.rs.html#46" class="src">Source</a>

``` rust
pub type ArrowRecordBatchStream = BoxStream<'static, Result<RecordBatch>>;
```

Expand description

A stream of arrow [`RecordBatch`](https://docs.rs/arrow-array/55.2.0/x86_64-unknown-linux-gnu/arrow_array/record_batch/struct.RecordBatch.html "struct arrow_array::record_batch::RecordBatch")es.

## Aliased Type<a href="https://docs.rs/iceberg/0.7.0/iceberg/scan/type.ArrowRecordBatchStream.html#aliased-type" class="anchor">§</a>

``` rust
pub struct ArrowRecordBatchStream { /* private fields */ }
```
