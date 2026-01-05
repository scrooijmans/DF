# Trait DeleteDyn Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/delete/api.rs.html#76-82" class="src">Source</a>

``` rust
pub trait DeleteDyn:
    Unpin
    + Send
    + Sync {
    // Required methods
    fn delete_dyn(&mut self, path: &str, args: OpDelete) -> Result<()>;
    fn flush_dyn(&mut self) -> BoxedFuture<'_, Result<usize>>;
}
```

Expand description

The dyn version of [`Delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html "trait opendal::raw::oio::Delete")

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html#tymethod.delete_dyn" class="fn">delete_dyn</a>(&mut self, path: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, args: <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.OpDelete.html" class="struct" title="struct opendal::raw::OpDelete">OpDelete</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

The dyn version of [`Delete::delete`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.delete "method opendal::raw::oio::Delete::delete")

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html#tymethod.flush_dyn" class="fn">flush_dyn</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html" class="type" title="type opendal::raw::BoxedFuture">BoxedFuture</a>\<'\_, <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>

The dyn version of [`Delete::flush`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html#tymethod.flush "method opendal::raw::oio::Delete::flush")

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html#impl-DeleteDyn-for-T" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Delete.html" class="trait" title="trait opendal::raw::oio::Delete">Delete</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.DeleteDyn.html" class="trait" title="trait opendal::raw::oio::DeleteDyn">DeleteDyn</a> for T
