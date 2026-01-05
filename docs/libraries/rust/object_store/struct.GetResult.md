# Struct GetResult Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/lib.rs.html#1042-1053" class="src">Source</a>

``` rust
pub struct GetResult {
    pub payload: GetResultPayload,
    pub meta: ObjectMeta,
    pub range: Range<u64>,
    pub attributes: Attributes,
}
```

Expand description

Result for a get request

## Fields<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#fields" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#structfield.payload" class="anchor field">§</a>`payload: `<a href="https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html" class="enum" title="enum object_store::GetResultPayload"><code>GetResultPayload</code></a>

The [`GetResultPayload`](https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html "enum object_store::GetResultPayload")

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#structfield.meta" class="anchor field">§</a>`meta: `<a href="https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta"><code>ObjectMeta</code></a>

The [`ObjectMeta`](https://docs.rs/object_store/latest/object_store/struct.ObjectMeta.html "struct object_store::ObjectMeta") for this object

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#structfield.range" class="anchor field">§</a>`range: `<a href="https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html" class="struct" title="struct core::ops::range::Range"><code>Range</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive"><code>u64</code></a>`>`

The range of bytes returned by this request

Note this is not `usize` as `object_store` supports 32-bit architectures such as WASM

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#structfield.attributes" class="anchor field">§</a>`attributes: `<a href="https://docs.rs/object_store/latest/object_store/struct.Attributes.html" class="struct" title="struct object_store::Attributes"><code>Attributes</code></a>

Additional object attributes

## Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#impl-GetResult" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>

#### pub async fn <a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#method.bytes" class="fn">bytes</a>(self) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>

Collects the data into a [`Bytes`](https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html "struct bytes::bytes::Bytes")

#### pub fn <a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#method.into_stream" class="fn">into_stream</a>(self) -\> <a href="https://docs.rs/futures-core/0.3.31/x86_64-unknown-linux-gnu/futures_core/stream/type.BoxStream.html" class="type" title="type futures_core::stream::BoxStream">BoxStream</a>\<'static, <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>\>

Converts this into a byte stream

If the `self.kind` is [`GetResultPayload::File`](https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#variant.File "variant object_store::GetResultPayload::File") will perform chunked reads of the file, otherwise will return the [`GetResultPayload::Stream`](https://docs.rs/object_store/latest/object_store/enum.GetResultPayload.html#variant.Stream "variant object_store::GetResultPayload::Stream").

##### <a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#tokio-compatibility" class="doc-anchor">§</a>Tokio Compatibility

Tokio discourages performing blocking IO on a tokio worker thread, however, no major operating systems have stable async file APIs. Therefore if called from a tokio context, this will use [`tokio::runtime::Handle::spawn_blocking`](https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/runtime/handle/struct.Handle.html#method.spawn_blocking "method tokio::runtime::handle::Handle::spawn_blocking") to dispatch IO to a blocking thread pool, much like `tokio::fs` does under-the-hood.

If not called from a tokio context, this will perform IO on the current thread with no additional complexity or overheads

## Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#impl-Debug-for-GetResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html" class="struct" title="struct object_store::GetResult">GetResult</a>

<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/object_store/latest/object_store/struct.GetResult.html#blanket-implementations" class="anchor">§</a>
