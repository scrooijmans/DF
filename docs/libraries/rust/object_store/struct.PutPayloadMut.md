# Struct PutPayloadMut Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/payload.rs.html#186-191" class="src">Source</a>

``` rust
pub struct PutPayloadMut { /* private fields */ }
```

Expand description

A builder for [`PutPayload`](https://docs.rs/object_store/latest/object_store/struct.PutPayload.html "struct object_store::PutPayload") that avoids reallocating memory

Data is allocated in fixed blocks, which are flushed to [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") once full. Unlike [`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") this avoids needing to repeatedly reallocate blocks of memory, which typically involves copying all the previously written data to a new contiguous memory region.

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#impl-PutPayloadMut" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html" class="struct" title="struct object_store::PutPayloadMut">PutPayloadMut</a>

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.new" class="fn">new</a>() -\> Self

Create a new [`PutPayloadMut`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html "struct object_store::PutPayloadMut")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.with_block_size" class="fn">with_block_size</a>(self, block_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Configures the minimum allocation size

Defaults to 8KB

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.extend_from_slice" class="fn">extend_from_slice</a>(&mut self, slice: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\])

Write bytes into this [`PutPayloadMut`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html "struct object_store::PutPayloadMut")

If there is an in-progress block, data will be first written to it, flushing it to [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") once full. If data remains to be written, a new block of memory of at least the configured block size will be allocated, to hold the remaining data.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.push" class="fn">push</a>(&mut self, bytes: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>)

Append a [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") to this [`PutPayloadMut`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html "struct object_store::PutPayloadMut") without copying

This will close any currently buffered block populated by [`Self::extend_from_slice`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.extend_from_slice "method object_store::PutPayloadMut::extend_from_slice"), and append `bytes` to this payload without copying.

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Returns `true` if this [`PutPayloadMut`](https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html "struct object_store::PutPayloadMut") contains no bytes

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.content_length" class="fn">content_length</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the total length of the [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes") in this payload

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.freeze" class="fn">freeze</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>

Convert into [`PutPayload`](https://docs.rs/object_store/latest/object_store/struct.PutPayload.html "struct object_store::PutPayload")

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#impl-Debug-for-PutPayloadMut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html" class="struct" title="struct object_store::PutPayloadMut">PutPayloadMut</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#impl-Default-for-PutPayloadMut" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html" class="struct" title="struct object_store::PutPayloadMut">PutPayloadMut</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#impl-From%3CPutPayloadMut%3E-for-PutPayload" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html" class="struct" title="struct object_store::PutPayloadMut">PutPayloadMut</a>\> for <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayload.html" class="struct" title="struct object_store::PutPayload">PutPayload</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(value: <a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html" class="struct" title="struct object_store::PutPayloadMut">PutPayloadMut</a>) -\> Self

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.PutPayloadMut.html#blanket-implementations" class="anchor">§</a>
