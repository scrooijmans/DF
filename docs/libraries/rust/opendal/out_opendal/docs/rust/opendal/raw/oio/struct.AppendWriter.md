# Struct AppendWriter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/append_write.rs.html#59-65" class="src">Source</a>

``` rust
pub struct AppendWriter<W: AppendWrite> { /* private fields */ }
```

Expand description

AppendWriter will implements [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on append object.

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#todo" class="doc-anchor">Â§</a>TODO

- Allow users to switch to un-buffered mode if users write 16MiB every time.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#impl-AppendWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html" class="trait" title="trait opendal::raw::oio::AppendWrite">AppendWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html" class="struct" title="struct opendal::raw::oio::AppendWriter">AppendWriter</a>\<W\>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#safety" class="doc-anchor">Â§</a>Safety

wasm32 is a special target that we only have one event-loop for this state.

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#method.new" class="fn">new</a>(inner: W) -\> Self

Create a new AppendWriter.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#impl-Write-for-AppendWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html" class="struct" title="struct opendal::raw::oio::AppendWriter">AppendWriter</a>\<W\>

where W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html" class="trait" title="trait opendal::raw::oio::AppendWrite">AppendWrite</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Write given bytes into writer. [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#method.close" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

Close the writer and make sure all data has been flushed.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#method.abort" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Abort the pending writer.

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html#blanket-implementations" class="anchor">Â§</a>
