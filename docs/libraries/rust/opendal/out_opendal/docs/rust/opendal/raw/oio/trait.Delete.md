# Trait Delete Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/delete/api.rs.html#30-57" class="src">Source</a>

``` rust
pub trait Delete:
    Unpin
    + Send
    + Sync {
    // Required methods
    fn delete(&mut self, path: &str, args: OpDelete) -> Result<()>;
    fn flush(&mut self) -> impl Future<Output = Result<usize>> + MaybeSend;
}
```

Expand description

The Delete trait defines interfaces for performing deletion operations.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete" class="fn">delete</a>(&mut self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Requests deletion of a resource at the specified path with optional arguments

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#parameters" class="doc-anchor">Â§</a>Parameters

- `path`: The path of the resource to delete
- `args`: Additional arguments for the delete operation

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#returns" class="doc-anchor">Â§</a>Returns

- `Ok(())`: The deletion request has been successfully queued (does not guarantee actual deletion)
- `Err(err)`: An error occurred and the deletion request was not queued

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#notes" class="doc-anchor">Â§</a>Notes

This method just queue the delete request. The actual deletion will be performed when `flush` is called.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Flushes the deletion queue to ensure queued deletions are executed

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#returns-1" class="doc-anchor">Â§</a>Returns

- `Ok(0)`: All queued deletions have been processed or the queue is empty.
- `Ok(count)`: The number of resources successfully deleted. Implementations should return an error if the queue is non-empty but no resources were deleted
- `Err(err)`: An error occurred while performing the deletions

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#notes-1" class="doc-anchor">Â§</a>Notes

- This method is asynchronous and will wait for queued deletions to complete

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#impl-Delete-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#method.delete" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete" class="fn">delete</a>(&mut self, \_: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, \_: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#method.flush" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#impl-Delete-for-Box%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html" class="trait" title="trait opendal::raw::oio::DeleteDyn">DeleteDyn</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#method.delete-1" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete" class="fn">delete</a>(&mut self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#method.flush-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#impl-Delete-for-BatchDeleter%3CD%3E" class="anchor">Â§</a>

### impl\<D: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html" class="trait" title="trait opendal::raw::oio::BatchDelete">BatchDelete</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html" class="struct" title="struct opendal::raw::oio::BatchDeleter">BatchDeleter</a>\<D\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#impl-Delete-for-OneShotDeleter%3CD%3E" class="anchor">Â§</a>

### impl\<D: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotDelete.html" class="trait" title="trait opendal::raw::oio::OneShotDelete">OneShotDelete</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html" class="struct" title="struct opendal::raw::oio::OneShotDeleter">OneShotDeleter</a>\<D\>
