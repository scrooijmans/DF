# Struct BatchDeleter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/delete/batch_delete.rs.html#59-62" class="src">Source</a>

``` rust
pub struct BatchDeleter<D: BatchDelete> { /* private fields */ }
```

Expand description

BatchDeleter is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on batch delete.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#impl-BatchDeleter%3CD%3E" class="anchor">Â§</a>

### impl\<D: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html" class="trait" title="trait opendal::raw::oio::BatchDelete">BatchDelete</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html" class="struct" title="struct opendal::raw::oio::BatchDeleter">BatchDeleter</a>\<D\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#method.new" class="fn">new</a>(inner: D) -\> Self

Create a new batch deleter.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#impl-Delete-for-BatchDeleter%3CD%3E" class="anchor">Â§</a>

### impl\<D: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BatchDelete.html" class="trait" title="trait opendal::raw::oio::BatchDelete">BatchDelete</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html" class="struct" title="struct opendal::raw::oio::BatchDeleter">BatchDeleter</a>\<D\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#method.delete" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete" class="fn">delete</a>(&mut self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Requests deletion of a resource at the specified path with optional arguments [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#method.flush" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Flushes the deletion queue to ensure queued deletions are executed [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BatchDeleter.html#blanket-implementations" class="anchor">Â§</a>
