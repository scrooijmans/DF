# Struct RecordBatchReceiverStreamBuilder Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/stream.rs.html#233" class="src">Source</a>

``` rust
pub struct RecordBatchReceiverStreamBuilder { /* private fields */ }
```

Expand description

Builder for `RecordBatchReceiverStream` that propagates errors and panic’s correctly.

[`RecordBatchReceiverStreamBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html "struct datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder") is used to spawn one or more tasks that produce [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es and send them to a single `Receiver` which can improve parallelism.

This also handles propagating panic\`s and canceling the tasks.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#example" class="doc-anchor">§</a>Example

The following example spawns 2 tasks that will write [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to the `tx` end of the builder, after building the stream, we can receive those batches with calling `.next()`

``` rust
let schema = Arc::new(Schema::new(vec![Field::new("foo", DataType::Int8, false)]));
let mut builder = RecordBatchReceiverStreamBuilder::new(Arc::clone(&schema), 10);

// task 1
let tx_1 = builder.tx();
let schema_1 = Arc::clone(&schema);
builder.spawn(async move {
    // Your task needs to send batches to the tx
    tx_1.send(Ok(RecordBatch::new_empty(schema_1))).await.unwrap();

    Ok(())
});

// task 2
let tx_2 = builder.tx();
let schema_2 = Arc::clone(&schema);
builder.spawn(async move {
    // Your task needs to send batches to the tx
    tx_2.send(Ok(RecordBatch::new_empty(schema_2))).await.unwrap();

    Ok(())
});

let mut stream = builder.build();
while let Some(res_batch) = stream.next().await {
    // `res_batch` can either from task 1 or 2

    // do something with `res_batch`
}
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#impl-RecordBatchReceiverStreamBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder">RecordBatchReceiverStreamBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.new" class="fn">new</a>( schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder">RecordBatchReceiverStreamBuilder</a>

Create new channels with the specified buffer size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.tx" class="fn">tx</a>(&self) -\> <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/sync/mpsc/bounded/struct.Sender.html" class="struct" title="struct tokio::sync::mpsc::bounded::Sender">Sender</a>\<<a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>

Get a handle for sending [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") to the output

If the stream is dropped / canceled, the sender will be closed and calling `tx().send()` will return an error. Producers should stop producing in this case and return control.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.spawn" class="fn">spawn</a>\<F\>(&mut self, task: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static,

Spawn task that will be aborted if this builder (or the stream built from it) are dropped

This is often used to spawn tasks that write to the sender retrieved from [`Self::tx`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.tx "method datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder::tx"), for examples, see the document of this type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.spawn_on" class="fn">spawn_on</a>\<F\>(&mut self, task: F, handle: &<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html" class="struct" title="struct tokio::runtime::handle::Handle">Handle</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static,

Same as [`Self::spawn`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.spawn "method datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder::spawn") but it spawns the task on the provided runtime.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.spawn_blocking" class="fn">spawn_blocking</a>\<F\>(&mut self, f: F)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static,

Spawn a blocking task tied to the builder and stream.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#drop--cancel-behavior" class="doc-anchor">§</a>Drop / Cancel Behavior

If this builder (or the stream built from it) is dropped **before** the task starts, the task is also dropped and will never start execute.

**Note:** Once the blocking task has started, it **will not** be forcibly stopped on drop as Rust does not allow forcing a running thread to terminate. The task will continue running until it completes or encounters an error.

Users should ensure that their blocking function periodically checks for errors calling `tx.blocking_send`. An error signals that the stream has been dropped / cancelled and the blocking task should exit.

This is often used to spawn tasks that write to the sender retrieved from [`Self::tx`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.tx "method datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder::tx"), for examples, see the document of this type.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.spawn_blocking_on" class="fn">spawn_blocking_on</a>\<F\>(&mut self, f: F, handle: &<a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html" class="struct" title="struct tokio::runtime::handle::Handle">Handle</a>)

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html" class="trait" title="trait core::ops::function::FnOnce">FnOnce</a>() -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static,

Same as [`Self::spawn_blocking`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.spawn_blocking "method datafusion::physical_plan::stream::RecordBatchReceiverStreamBuilder::spawn_blocking") but it spawns the blocking task on the provided runtime.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#method.build" class="fn">build</a>( self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>\>

Create a stream of all [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch") written to `tx`

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchReceiverStreamBuilder.html#blanket-implementations" class="anchor">§</a>
