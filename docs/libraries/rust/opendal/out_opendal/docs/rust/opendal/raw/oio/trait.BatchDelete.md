# Trait BatchDelete Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/delete/batch_delete.rs.html#27-47" class="src">Source</a>

``` rust
pub trait BatchDelete:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required methods
    fn delete_once(
        &self,
        path: String,
        args: OpDelete,
    ) -> impl Future<Output = Result<()>> + MaybeSend;
    fn delete_batch(
        &self,
        batch: Vec<(String, OpDelete)>,
    ) -> impl Future<Output = Result<BatchDeleteResult>> + MaybeSend;
}
```

Expand description

BatchDelete is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on batch delete operation.

OneShotDeleter will perform delete operation while calling `flush`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html#tymethod.delete_once" class="fn">delete_once</a>( &self, path: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

delete_once delete one path at once.

Implementations should make sure that the data is deleted correctly at once.

BatchDeleter may call this method while there are only one path to delete.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html#tymethod.delete_batch" class="fn">delete_batch</a>( &self, batch: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>)\>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleteResult.html" class="struct" title="struct opendal::raw::oio::BatchDeleteResult">BatchDeleteResult</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

delete_batch delete multiple paths at once.

- Implementations should make sure that the length of `batch` equals to the return resultâ€™s length.
- Implementations should return error no path is deleted.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html#implementors" class="anchor">Â§</a>
