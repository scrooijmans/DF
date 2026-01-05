# Trait OneShotDelete Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/delete/one_shot_delete.rs.html#26-35" class="src">Source</a>

``` rust
pub trait OneShotDelete:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required method
    fn delete_once(
        &self,
        path: String,
        args: OpDelete,
    ) -> impl Future<Output = Result<()>> + MaybeSend;
}
```

Expand description

OneShotDelete is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on one shot operation.

OneShotDeleter will perform delete operation while calling `flush`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotDelete.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotDelete.html#tymethod.delete_once" class="fn">delete_once</a>( &self, path: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

delete_once delete one path at once.

Implementations should make sure that the data is deleted correctly at once.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotDelete.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotDelete.html#implementors" class="anchor">Â§</a>
