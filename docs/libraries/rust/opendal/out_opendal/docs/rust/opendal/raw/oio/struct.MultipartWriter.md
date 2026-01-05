# Struct MultipartWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/multipart_write.rs.html#133-143" class="src">Source</a>

``` rust
pub struct MultipartWriter<W: MultipartWrite> { /* private fields */ }
```

Expand description

MultipartWriter will implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on multipart uploads.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#impl-MultipartWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.MultipartWrite.html" class="trait" title="trait opendal::raw::oio::MultipartWrite">MultipartWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html" class="struct" title="struct opendal::raw::oio::MultipartWriter">MultipartWriter</a>\<W\>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#safety" class="doc-anchor">Â§</a>Safety

wasm32 is a special target that we only have one event-loop for this state.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#method.new" class="fn">new</a>(info: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>, inner: W, concurrent: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new MultipartWriter.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#impl-Write-for-MultipartWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html" class="struct" title="struct opendal::raw::oio::MultipartWriter">MultipartWriter</a>\<W\>

where W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.MultipartWrite.html" class="trait" title="trait opendal::raw::oio::MultipartWrite">MultipartWrite</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write given bytes into writer. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#method.close" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Close the writer and make sure all data has been flushed.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#method.abort" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort the pending writer.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html#blanket-implementations" class="anchor">Â§</a>
