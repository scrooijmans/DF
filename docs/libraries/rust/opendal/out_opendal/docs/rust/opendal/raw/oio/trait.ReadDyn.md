# Trait ReadDyn Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/read/api.rs.html#87-95" class="src">Source</a>

``` rust
pub trait ReadDyn:
    Unpin
    + Send
    + Sync {
    // Required methods
    fn read_dyn(&mut self) -> BoxedFuture<'_, Result<Buffer>>;
    fn read_all_dyn(&mut self) -> BoxedFuture<'_, Result<Buffer>>;
}
```

Expand description

ReadDyn is the dyn version of [`Read`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html "trait opendal::raw::oio::Read") make it possible to use as `Box<dyn ReadDyn>`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html#tymethod.read_dyn" class="fn">read_dyn</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\>

The dyn version of [`Read::read`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read "method opendal::raw::oio::Read::read").

This function returns a boxed future to make it object safe.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html#tymethod.read_all_dyn" class="fn">read_all_dyn</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\>

The dyn version of [`Read::read_all`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all "method opendal::raw::oio::Read::read_all")

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html#impl-ReadDyn-for-T" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html" class="trait" title="trait opendal::raw::oio::ReadDyn">ReadDyn</a> for T
