# Struct OneShotWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/one_shot_write.rs.html#37-40" class="src">Source</a>

``` rust
pub struct OneShotWriter<W: OneShotWrite> { /* private fields */ }
```

Expand description

OneShotWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on one shot.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#impl-OneShotWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html" class="trait" title="trait opendal::raw::oio::OneShotWrite">OneShotWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html" class="struct" title="struct opendal::raw::oio::OneShotWriter">OneShotWriter</a>\<W\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#method.new" class="fn">new</a>(inner: W) -\> Self

Create a new one shot writer.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#impl-Write-for-OneShotWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html" class="trait" title="trait opendal::raw::oio::OneShotWrite">OneShotWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html" class="struct" title="struct opendal::raw::oio::OneShotWriter">OneShotWriter</a>\<W\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write given bytes into writer. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#method.close" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Close the writer and make sure all data has been flushed.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#method.abort" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort the pending writer.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html#blanket-implementations" class="anchor">Â§</a>
