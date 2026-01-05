# Trait OneShotWrite Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/one_shot_write.rs.html#29-34" class="src">Source</a>

``` rust
pub trait OneShotWrite:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required method
    fn write_once(
        &self,
        bs: Buffer,
    ) -> impl Future<Output = Result<Metadata>> + MaybeSend;
}
```

Expand description

OneShotWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on one shot operation. By implementing OneShotWrite, services donâ€™t need to care about the details.

For example, S3 `PUT Object` and fs `write_all`.

The layout after adopting [`OneShotWrite`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html "trait opendal::raw::oio::OneShotWrite"):

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html#tymethod.write_once" class="fn">write_once</a>( &self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

write_once write all data at once.

Implementations should make sure that the data is written correctly at once.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html#implementors" class="anchor">Â§</a>
