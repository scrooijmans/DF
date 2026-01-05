# Struct MemoryConsumer Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/memory_pool/mod.rs.html#239" class="src">Source</a>

``` rust
pub struct MemoryConsumer { /* private fields */ }
```

Expand description

A memory consumer is a named allocation traced by a particular [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") in a [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool"). All allocations are registered to a particular `MemoryConsumer`;

Each `MemoryConsumer` is identifiable by a process-unique id, and is therefor not cloneable, If you want a clone of a `MemoryConsumer`, you should look into [`MemoryConsumer::clone_with_new_id`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.clone_with_new_id "method datafusion::execution::memory_pool::MemoryConsumer::clone_with_new_id"), but note that this `MemoryConsumer` may be treated as a separate entity based on the used pool, and is only guaranteed to share the name and inner properties.

For help with allocation accounting, see the [`proxy`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/proxy/index.html "mod datafusion::execution::memory_pool::proxy") module.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#impl-MemoryConsumer" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.new" class="fn">new</a>(name: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

Create a new empty [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") that can be grown using [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.clone_with_new_id" class="fn">clone_with_new_id</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

Returns a clone of this [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") with a new unique id, which can be registered with a [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool"), This new consumer is separate from the original.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.id" class="fn">id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Return the unique id of this [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.with_can_spill" class="fn">with_can_spill</a>(self, can_spill: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

Set whether this allocation can be spilled to disk

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.can_spill" class="fn">can_spill</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns true if this allocation can spill to disk

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.name" class="fn">name</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Returns the name associated with this allocation

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.register" class="fn">register</a>(self, pool: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryReservation">MemoryReservation</a>

Registers this [`MemoryConsumer`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html "struct datafusion::execution::memory_pool::MemoryConsumer") with the provided [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") returning a [`MemoryReservation`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryReservation.html "struct datafusion::execution::memory_pool::MemoryReservation") that can be used to grow or shrink the memory reservation

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#impl-Debug-for-MemoryConsumer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#impl-Hash-for-MemoryConsumer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#impl-PartialEq-for-MemoryConsumer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#impl-Eq-for-MemoryConsumer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html" class="struct" title="struct datafusion::execution::memory_pool::MemoryConsumer">MemoryConsumer</a>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.MemoryConsumer.html#blanket-implementations" class="anchor">§</a>
