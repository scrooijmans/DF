# Struct OneShotDeleter Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/delete/one_shot_delete.rs.html#38-41" class="src">Source</a>

``` rust
pub struct OneShotDeleter<D> { /* private fields */ }
```

Expand description

OneShotDelete is used to implement [`oio::Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete") based on one shot.

## Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#impl-OneShotDeleter%3CD%3E" class="anchor">Â§</a>

### impl\<D\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html" class="struct" title="struct opendal::raw::oio::OneShotDeleter">OneShotDeleter</a>\<D\>

#### pub fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#method.new" class="fn">new</a>(inner: D) -\> Self

Create a new one shot deleter.

## Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#trait-implementations" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#impl-Delete-for-OneShotDeleter%3CD%3E" class="anchor">Â§</a>

### impl\<D: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotDelete.html" class="trait" title="trait opendal::raw::oio::OneShotDelete">OneShotDelete</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html" class="struct" title="struct opendal::raw::oio::OneShotDeleter">OneShotDeleter</a>\<D\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#method.delete" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete" class="fn">delete</a>(&mut self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Requests deletion of a resource at the specified path with optional arguments [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete)

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#method.flush" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Flushes the deletion queue to ensure queued deletions are executed [Read more](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush)

## Auto Trait Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#synthetic-implementations" class="anchor">Â§</a>

## Blanket Implementations<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotDeleter.html#blanket-implementations" class="anchor">Â§</a>
