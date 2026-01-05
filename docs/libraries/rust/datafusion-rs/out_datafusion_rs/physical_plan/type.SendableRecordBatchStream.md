# Type Alias SendableRecordBatchStream Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/stream.rs.html#60" class="src">Source</a>

``` rust
pub type SendableRecordBatchStream = Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>;
```

Expand description

Trait for a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") of [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es that can be passed between threads

This trait is used to retrieve the results of DataFusion execution plan nodes.

The trait is a specialized Rust Async [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") that also knows the schema of the data it will return (even if the stream has no data). Every `RecordBatch` returned by the stream should have the same schema as returned by [`schema`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema "method datafusion::execution::RecordBatchStream::schema").

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/type.SendableRecordBatchStream.html#see-also" class="doc-anchor">§</a>See Also

- [`RecordBatchStreamAdapter`](https://docs.rs/datafusion/latest/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html) to convert an existing [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") to [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/type.SendableRecordBatchStream.html#error-handling" class="doc-anchor">§</a>Error Handling

Once a stream returns an error, it should not be polled again (the caller should stop calling `next`) and handle the error.

However, returning `Ready(None)` (end of stream) is likely the safest behavior after an error. Like [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")s, `RecordBatchStream`s should not be polled after end of stream or returning an error. However, also like [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream")s there is no mechanism to prevent callers polling so returning `Ready(None)` is recommended.

## Aliased Type<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/type.SendableRecordBatchStream.html#aliased-type" class="anchor">§</a>

``` rust
pub struct SendableRecordBatchStream { /* private fields */ }
```
