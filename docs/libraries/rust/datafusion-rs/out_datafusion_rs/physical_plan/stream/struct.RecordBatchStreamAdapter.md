# Struct RecordBatchStreamAdapter Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/stream.rs.html#395-406" class="src">Source</a>

``` rust
pub struct RecordBatchStreamAdapter<S> { /* private fields */ }
```

Expand description

Combines a [`Stream`](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html "trait futures_core::stream::Stream") with a [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef") implementing [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream") for the combination

See [`Self::new`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#method.new "associated function datafusion::physical_plan::stream::RecordBatchStreamAdapter::new") for an example

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#impl-RecordBatchStreamAdapter%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#method.new" class="fn">new</a>(schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, stream: S) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>

Creates a new [`RecordBatchStreamAdapter`](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html "struct datafusion::physical_plan::stream::RecordBatchStreamAdapter") from the provided schema and stream.

Note to create a [`SendableRecordBatchStream`](https://docs.rs/datafusion/50.2.0/datafusion/execution/type.SendableRecordBatchStream.html "type datafusion::execution::SendableRecordBatchStream") you pin the result

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#example" class="doc-anchor">§</a>Example

``` rust
// Create stream of Result<RecordBatch>
let batch = record_batch!(
  ("a", Int32, [1, 2, 3]),
  ("b", Float64, [Some(4.0), None, Some(5.0)])
).expect("created batch");
let schema = batch.schema();
let stream = futures::stream::iter(vec![Ok(batch)]);
// Convert the stream to a SendableRecordBatchStream
let adapter = RecordBatchStreamAdapter::new(schema, stream);
// Now you can use the adapter as a SendableRecordBatchStream
let batch_stream: SendableRecordBatchStream = Box::pin(adapter);
// ...
```

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#impl-Debug-for-RecordBatchStreamAdapter%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#impl-RecordBatchStream-for-RecordBatchStreamAdapter%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>

where S: <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the schema of this `RecordBatchStream`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#impl-Stream-for-RecordBatchStreamAdapter%3CS%3E" class="anchor">§</a>

### impl\<S\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>

where S: <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Values yielded by the stream.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#method.poll_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next" class="fn">poll_next</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\> as <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\>::<a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype" title="type futures_core::stream::Stream::Item">Item</a>\>\>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#method.size_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint" class="fn">size_hint</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the stream. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#impl-Unpin-for-RecordBatchStreamAdapter%3CS%3E" class="anchor">§</a>

### impl\<'\_\_pin, S\> <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html" class="struct" title="struct datafusion::physical_plan::stream::RecordBatchStreamAdapter">RecordBatchStreamAdapter</a>\<S\>

where \<PinnedFieldsOfHelperStruct\<\_\_Origin\<'\_\_pin, S\>\> as PinnedFieldsOfHelperTrait\>::Actual: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a>,

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/stream/struct.RecordBatchStreamAdapter.html#blanket-implementations" class="anchor">§</a>
