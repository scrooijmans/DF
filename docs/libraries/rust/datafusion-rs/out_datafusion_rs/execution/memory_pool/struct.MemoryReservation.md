# Struct MemoryReservation Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/memory_pool/mod.rs.html#351" class="src">Source</a>

``` rust
pub struct MemoryReservation { /* private fields */ }
```

Expand description

A [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") tracks an individual reservation of a number of bytes of memory in a [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") that is freed back to the pool on drop.

The reservation can be grown or shrunk over time.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#impl-MemoryReservation" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.size" class="fn">size</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the size of this reservation in bytes

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.consumer" class="fn">consumer</a>(&self) -\> &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

Returns [MemoryConsumer](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") for this [MemoryReservation](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.free" class="fn">free</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Frees all bytes from this reservation back to the underlying pool, returning the number of bytes freed.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.shrink" class="fn">shrink</a>(&mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Frees `capacity` bytes from this reservation

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#panics" class="doc-anchor">§</a>Panics

Panics if `capacity` exceeds [`Self::size`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.size "method datafusion::execution::memory_pool::MemoryReservation::size")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.try_shrink" class="fn">try_shrink</a>(&mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Tries to free `capacity` bytes from this reservation if `capacity` does not exceed [`Self::size`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.size "method datafusion::execution::memory_pool::MemoryReservation::size") Returns new reservation size or error if shrinking capacity is more than allocated size

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.resize" class="fn">resize</a>(&mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Sets the size of this reservation to `capacity`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.try_resize" class="fn">try_resize</a>(&mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to set the size of this reservation to `capacity`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.grow" class="fn">grow</a>(&mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Increase the size of this reservation by `capacity` bytes

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.try_grow" class="fn">try_grow</a>(&mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Try to increase the size of this reservation by `capacity` bytes, returning error if there is insufficient capacity left in the pool.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.split" class="fn">split</a>(&mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>

Splits off `capacity` bytes from this [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") into a new [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") with the same [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer").

This can be useful to free part of this reservation with RAAI style dropping

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#panics-1" class="doc-anchor">§</a>Panics

Panics if `capacity` exceeds [`Self::size`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.size "method datafusion::execution::memory_pool::MemoryReservation::size")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.new_empty" class="fn">new_empty</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>

Returns a new empty [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") with the same [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.take" class="fn">take</a>(&mut self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>

Splits off all the bytes from this [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") into a new [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") with the same [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer")

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#impl-Debug-for-MemoryReservation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#impl-Drop-for-MemoryReservation" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html#blanket-implementations" class="anchor">§</a>
