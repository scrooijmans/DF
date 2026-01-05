# Trait WriteDyn Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/api.rs.html#66-75" class="src">Source</a>

``` rust
pub trait WriteDyn:
    Unpin
    + Send
    + Sync {
    // Required methods
    fn write_dyn(&mut self, bs: Buffer) -> BoxedFuture<'_, Result<()>>;
    fn close_dyn(&mut self) -> BoxedFuture<'_, Result<Metadata>>;
    fn abort_dyn(&mut self) -> BoxedFuture<'_, Result<()>>;
}
```

Expand description

WriteDyn is the dyn version of [`Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") make it possible to use as `Box<dyn WriteDyn>`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html#tymethod.write_dyn" class="fn">write_dyn</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

The dyn version of [`Write::write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write "method opendal::raw::oio::Write::write").

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html#tymethod.close_dyn" class="fn">close_dyn</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>\>

The dyn version of [`Write::close`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close "method opendal::raw::oio::Write::close").

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html#tymethod.abort_dyn" class="fn">abort_dyn</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\>

The dyn version of [`Write::abort`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort "method opendal::raw::oio::Write::abort").

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html#impl-WriteDyn-for-T" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html" class="trait" title="trait opendal::raw::oio::WriteDyn">WriteDyn</a> for T
