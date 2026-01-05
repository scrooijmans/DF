# Trait PartitionStream Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/streaming.rs.html#50" class="src">Source</a>

``` rust
pub trait PartitionStream:
    Debug
    + Send
    + Sync {
    // Required methods
    fn schema(&self) -> &Arc<Schema>;
    fn execute(
        &self,
        ctx: Arc<TaskContext>,
    ) -> Pin<Box<dyn RecordBatchStream<Item = Result<RecordBatch, DataFusionError>> + Send>>;
}
```

Expand description

A partition that can be converted into a [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream")

Combined with [`StreamingTableExec`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/struct.StreamingTableExec.html "struct datafusion::physical_plan::streaming::StreamingTableExec"), you can use this trait to implement [`ExecutionPlan`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.ExecutionPlan.html "trait datafusion::physical_plan::ExecutionPlan") for a custom source with less boiler plate than implementing `ExecutionPlan` directly for many use cases.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html#tymethod.schema" class="fn">schema</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the schema of this partition

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html#tymethod.execute" class="fn">execute</a>( &self, ctx: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.TaskContext.html" class="struct" title="struct datafusion::execution::TaskContext">TaskContext</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>

Returns a stream yielding this partitions values

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html#impl-PartitionStream-for-TestPartitionStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/streaming/trait.PartitionStream.html" class="trait" title="trait datafusion::physical_plan::streaming::PartitionStream">PartitionStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/struct.TestPartitionStream.html" class="struct" title="struct datafusion::physical_plan::test::TestPartitionStream">TestPartitionStream</a>
