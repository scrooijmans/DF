# Struct EmptyRecordBatchStream Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/stream.rs.html#470" class="src">Source</a>

``` rust
pub struct EmptyRecordBatchStream { /* private fields */ }
```

Expand description

`EmptyRecordBatchStream` can be used to create a [`RecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html "trait datafusion::execution::RecordBatchStream") that will produce no results

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#impl-EmptyRecordBatchStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#method.new" class="fn">new</a>(schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>

Create an empty RecordBatchStream

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#impl-RecordBatchStream-for-EmptyRecordBatchStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the schema of this `RecordBatchStream`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#impl-Stream-for-EmptyRecordBatchStream" class="anchor">§</a>

### impl <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Values yielded by the stream.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#method.poll_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next" class="fn">poll_next</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a>\>, \_cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html" class="struct" title="struct datafusion::physical_plan::EmptyRecordBatchStream">EmptyRecordBatchStream</a> as <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\>::<a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype" title="type futures_core::stream::Stream::Item">Item</a>\>\>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#method.size_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint" class="fn">size_hint</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the stream. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/struct.EmptyRecordBatchStream.html#blanket-implementations" class="anchor">§</a>
