# Struct PositionWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/position_write.rs.html#70-77" class="src">Source</a>

``` rust
pub struct PositionWriter<W: PositionWrite> { /* private fields */ }
```

Expand description

PositionWriter will implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on position write.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#impl-PositionWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html" class="trait" title="trait opendal::raw::oio::PositionWrite">PositionWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html" class="struct" title="struct opendal::raw::oio::PositionWriter">PositionWriter</a>\<W\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#method.new" class="fn">new</a>(info: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.AccessorInfo.html" class="struct" title="struct opendal::raw::AccessorInfo">AccessorInfo</a>\>, inner: W, concurrent: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Create a new PositionWriter.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#impl-Write-for-PositionWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html" class="trait" title="trait opendal::raw::oio::PositionWrite">PositionWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html" class="struct" title="struct opendal::raw::oio::PositionWriter">PositionWriter</a>\<W\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write given bytes into writer. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#method.close" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Close the writer and make sure all data has been flushed.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#method.abort" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort the pending writer.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html#blanket-implementations" class="anchor">Â§</a>
