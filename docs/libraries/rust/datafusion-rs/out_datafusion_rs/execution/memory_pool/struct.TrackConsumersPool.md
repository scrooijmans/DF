# Struct TrackConsumersPool Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/memory_pool/pool.rs.html#325" class="src">Source</a>

``` rust
pub struct TrackConsumersPool<I> { /* private fields */ }
```

Expand description

A [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that tracks the consumers that have reserved memory within the inner memory pool.

By tracking memory reservations more carefully this pool can provide better error messages on the largest memory users when memory allocation fails.

Tracking is per hashed [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer"), not per [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation"). The same consumer can have multiple reservations.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#automatic-usage-via-runtimeenvbuilder" class="doc-anchor">§</a>Automatic Usage via [`RuntimeEnvBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html "struct datafusion::execution::runtime_env::RuntimeEnvBuilder")

The easiest way to use `TrackConsumersPool` is via [`RuntimeEnvBuilder::with_memory_limit()`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_memory_limit "method datafusion::execution::runtime_env::RuntimeEnvBuilder::with_memory_limit").

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#usage-examples" class="doc-anchor">§</a>Usage Examples

For more examples of using `TrackConsumersPool`, see the [memory_pool_tracking.rs](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/memory_pool_tracking.rs) example

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#impl-TrackConsumersPool%3CI%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html" class="struct" title="struct datafusion::execution::memory_pool::TrackConsumersPool">TrackConsumersPool</a>\<I\>

where I: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a>,

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.new" class="fn">new</a>(inner: I, top: <a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html" class="struct" title="struct datafusion::execution::memory_pool::TrackConsumersPool">TrackConsumersPool</a>\<I\>

Creates a new [`TrackConsumersPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html "struct datafusion::execution::memory_pool::TrackConsumersPool").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#arguments" class="doc-anchor">§</a>Arguments

- `inner` - The underlying memory pool that handles actual memory allocation
- `top` - The number of top memory consumers to include in error messages

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#note" class="doc-anchor">§</a>Note

In most cases, you should use [`RuntimeEnvBuilder::with_memory_limit()`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_memory_limit "method datafusion::execution::runtime_env::RuntimeEnvBuilder::with_memory_limit") instead of creating this pool manually, as it automatically sets up tracking with sensible defaults (top 5 consumers).

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#example" class="doc-anchor">§</a>Example

``` rust
use std::num::NonZeroUsize;
use datafusion_execution::memory_pool::{TrackConsumersPool, GreedyMemoryPool, FairSpillPool};

// Create with a greedy pool backend, reporting top 3 consumers in error messages
let tracked_greedy = TrackConsumersPool::new(
    GreedyMemoryPool::new(1024 * 1024), // 1MB limit
    NonZeroUsize::new(3).unwrap(),
);

// Create with a fair spill pool backend, reporting top 5 consumers in error messages
let tracked_fair = TrackConsumersPool::new(
    FairSpillPool::new(2 * 1024 * 1024), // 2MB limit
    NonZeroUsize::new(5).unwrap(),
);
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#impact-on-error-messages" class="doc-anchor">§</a>Impact on Error Messages

The `top` determines how many Top K [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer")s to include in the reported [`DataFusionError::ResourcesExhausted`](https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html#variant.ResourcesExhausted "variant datafusion::error::DataFusionError::ResourcesExhausted").

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.report_top" class="fn">report_top</a>(&self, top: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Returns a formatted string with the top memory consumers.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#impl-Debug-for-TrackConsumersPool%3CI%3E" class="anchor">§</a>

### impl\<I\> <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html" class="struct" title="struct datafusion::execution::memory_pool::TrackConsumersPool">TrackConsumersPool</a>\<I\>

where I: <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#impl-MemoryPool-for-TrackConsumersPool%3CI%3E" class="anchor">§</a>

### impl\<I\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html" class="struct" title="struct datafusion::execution::memory_pool::TrackConsumersPool">TrackConsumersPool</a>\<I\>

where I: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a>,

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.register" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.register" class="fn">register</a>(&self, consumer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>)

Registers a new [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.register)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.unregister" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.unregister" class="fn">unregister</a>(&self, consumer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>)

Records the destruction of a [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") with [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.unregister)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.grow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.grow" class="fn">grow</a>(&self, reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Infallibly grow the provided `reservation` by `additional` bytes [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.grow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.shrink" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.shrink" class="fn">shrink</a>(&self, reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, shrink: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Infallibly shrink the provided `reservation` by `shrink` bytes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.try_grow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.try_grow" class="fn">try_grow</a>( &self, reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Attempt to grow the provided `reservation` by `additional` bytes [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.try_grow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.reserved" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.reserved" class="fn">reserved</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the total amount of memory reserved

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#method.memory_limit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.memory_limit" class="fn">memory_limit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html" class="enum" title="enum datafusion::execution::memory_pool::MemoryLimit">MemoryLimit</a>

Return the memory limit of the pool [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.memory_limit)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html#blanket-implementations" class="anchor">§</a>
