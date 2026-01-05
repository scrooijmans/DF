# Struct MemoryStream Copy item path

<a href="https://docs.rs/datafusion-physical-plan/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_physical_plan/memory.rs.html#45" class="src">Source</a>

``` rust
pub struct MemoryStream { /* private fields */ }
```

Expand description

Iterator over batches

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#impl-MemoryStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#method.try_new" class="fn">try_new</a>( data: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>\>, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create an iterator for a vector of record batches

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#method.with_reservation" class="fn">with_reservation</a>(self, reservation: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>

Set the memory reservation for the data

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#method.with_fetch" class="fn">with_fetch</a>(self, fetch: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>

Set the number of rows to produce

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#impl-RecordBatchStream-for-MemoryStream" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html" class="trait" title="trait datafusion::execution::RecordBatchStream">RecordBatchStream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/trait.RecordBatchStream.html#tymethod.schema" class="fn">schema</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>

Get the schema

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#impl-Stream-for-MemoryStream" class="anchor">§</a>

### impl <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#associatedtype.Item" class="anchor">§</a>

#### type <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype">Item</a> = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Values yielded by the stream.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#method.poll_next" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next" class="fn">poll_next</a>( self: <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<&mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a>\>, \_: &mut <a href="https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html" class="struct" title="struct core::task::wake::Context">Context</a>\<'\_\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html" class="enum" title="enum core::task::poll::Poll">Poll</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html" class="struct" title="struct datafusion::physical_plan::memory::MemoryStream">MemoryStream</a> as <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html" class="trait" title="trait futures_core::stream::Stream">Stream</a>\>::<a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#associatedtype.Item" class="associatedtype" title="type futures_core::stream::Stream::Item">Item</a>\>\>

Attempt to pull out the next value of this stream, registering the current task for wakeup if the value is not yet available, and returning `None` if the stream is exhausted. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#tymethod.poll_next)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#method.size_hint" class="anchor">§</a>

#### fn <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint" class="fn">size_hint</a>(&self) -\> (<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>)

Returns the bounds on the remaining length of the stream. [Read more](https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/trait.Stream.html#method.size_hint)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_plan/memory/struct.MemoryStream.html#blanket-implementations" class="anchor">§</a>
