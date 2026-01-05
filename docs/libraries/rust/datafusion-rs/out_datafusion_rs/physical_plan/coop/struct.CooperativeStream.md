# Struct CooperativeStream Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/coop.rs.html#100" class="src">Source</a>

``` rust
pub struct CooperativeStream<T>where
    T: RecordBatchStream + Unpin,{ /* private fields */ }
```

Expand description

A stream that passes record batches through unchanged while cooperating with the Tokio runtime. It consumes cooperative scheduling budget for each returned [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch"), allowing other tasks to execute when the budget is exhausted.

See the [module level documentation](https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/index.html "mod datafusion::physical_plan::coop") for an in-depth discussion.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#impl-CooperativeStream%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#method.new" class="fn">new</a>(inner: T) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>\<T\>

Creates a new `CooperativeStream` that wraps the provided stream. The resulting stream will cooperate with the Tokio scheduler by consuming a unit of scheduling budget when the wrapped `Stream` returns a record batch.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#impl-RecordBatchStream-for-CooperativeStream%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Returns the schema of this `RecordBatchStream`. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#impl-Stream-for-CooperativeStream%3CT%3E" class="anchor">§</a>

### impl\<T\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>\<T\>

where T: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Values yielded by the stream.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#method.poll_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next" class="fn">poll_next</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>\<T\>\>, cx: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html" class="struct" title="struct datafusion::physical_plan::coop::CooperativeStream">CooperativeStream</a>\<T\> as <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\>::<a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype" title="type futures_core::stream::Stream::Item">Item</a>\>\>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#method.size_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint" class="fn">size_hint</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the stream. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/coop/struct.CooperativeStream.html#blanket-implementations" class="anchor">§</a>
