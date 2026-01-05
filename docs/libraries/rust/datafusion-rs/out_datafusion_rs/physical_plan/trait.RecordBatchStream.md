# Trait RecordBatchStream Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/stream.rs.html#26" class="src">Source</a>

``` rust
pub trait RecordBatchStream: Stream<Item = Result<RecordBatch, DataFusionError>> {
    // Required method
    fn schema(&self) -> Arc<Schema>;
}
```

Expand description

Trait for types that stream [RecordBatch](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

See [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream") for more details.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the schema of this `RecordBatchStream`.

Implementation of this trait should guarantee that all `RecordBatch`’s returned by this stream should have the same schema as returned from this method.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-FileStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/physical_plan/struct.FileStream.html" class="struct" title="struct datafusion::datasource::physical_plan::FileStream">FileStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-LimitStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/limit/struct.LimitStream.html" class="struct" title="struct datafusion::physical_plan::limit::LimitStream">LimitStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-LazyMemoryStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.LazyMemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::LazyMemoryStream">LazyMemoryStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-MemoryStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-BatchSplitStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.BatchSplitStream.html" class="struct" title="struct datafusion::physical_plan::stream::BatchSplitStream">BatchSplitStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-EmptyRecordBatchStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-BlockingStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.BlockingStream.html" class="struct" title="struct datafusion::physical_plan::test::exec::BlockingStream">BlockingStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-TestStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/test/exec/struct.TestStream.html" class="struct" title="struct datafusion::physical_plan::test::exec::TestStream">TestStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-RecordBatchStreamAdapter%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>

where S: <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/trait.RecordBatchStream.html#impl-RecordBatchStream-for-CooperativeStream%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a>,
