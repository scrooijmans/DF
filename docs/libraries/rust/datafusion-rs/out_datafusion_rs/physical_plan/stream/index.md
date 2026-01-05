# Module stream Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/lib.rs.html#85" class="src">Source</a>

Expand description

Stream wrappers for physical operators

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.BatchSplitStream.html" class="struct" title="struct datafusion::physical_plan::stream::BatchSplitStream">BatchSplitStream</a>  
Stream wrapper that splits large [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es into smaller batches.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::stream::EmptyRecordBatchStream">EmptyRecordBatchStream</a>  
`EmptyRecordBatchStream` can be used to create a [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream") that will produce no results

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder">RecordBatchReceiverStreamBuilder</a>  
Builder for `RecordBatchReceiverStream` that propagates errors and panic’s correctly.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>  
Combines a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") with a [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef") implementing [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream") for the combination
