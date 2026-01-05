# Struct UnboundedMemoryPool Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/memory_pool/pool.rs.html#32" class="src">Source</a>

``` rust
pub struct UnboundedMemoryPool { /* private fields */ }
```

Expand description

A [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that enforces no limit

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#impl-Debug-for-UnboundedMemoryPool" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::UnboundedMemoryPool">UnboundedMemoryPool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#impl-Default-for-UnboundedMemoryPool" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::UnboundedMemoryPool">UnboundedMemoryPool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::UnboundedMemoryPool">UnboundedMemoryPool</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#impl-MemoryPool-for-UnboundedMemoryPool" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html" class="struct" title="struct datafusion::execution::memory_pool::UnboundedMemoryPool">UnboundedMemoryPool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.grow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.grow" class="fn">grow</a>(&self, \_reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Infallibly grow the provided `reservation` by `additional` bytes [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.grow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.shrink" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.shrink" class="fn">shrink</a>(&self, \_reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, shrink: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Infallibly shrink the provided `reservation` by `shrink` bytes

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.try_grow" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.try_grow" class="fn">try_grow</a>( &self, reservation: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>, additional: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Attempt to grow the provided `reservation` by `additional` bytes [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.try_grow)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.reserved" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#tymethod.reserved" class="fn">reserved</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the total amount of memory reserved

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.memory_limit" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.memory_limit" class="fn">memory_limit</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/enum.MemoryLimit.html" class="enum" title="enum datafusion::execution::memory_pool::MemoryLimit">MemoryLimit</a>

Return the memory limit of the pool [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.memory_limit)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.register" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.register" class="fn">register</a>(&self, \_consumer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>)

Registers a new [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.register)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#method.unregister" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.unregister" class="fn">unregister</a>(&self, \_consumer: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>)

Records the destruction of a [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") with [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") [Read more](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html#method.unregister)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html#blanket-implementations" class="anchor">§</a>
