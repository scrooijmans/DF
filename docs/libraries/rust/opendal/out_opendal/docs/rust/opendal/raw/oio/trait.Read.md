# Trait Read Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/read/api.rs.html#44-62" class="src">Source</a>

``` rust
pub trait Read:
    Unpin
    + Send
    + Sync {
    // Required method
    fn read(&mut self) -> impl Future<Output = Result<Buffer>> + MaybeSend;

    // Provided method
    fn read_all(&mut self) -> impl Future<Output = Result<Buffer>> + MaybeSend { ... }
}
```

Expand description

Read is the internal trait used by OpenDAL to read data from storage.

Users should not use or import this trait unless they are implementing an `Accessor`.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#notes" class="doc-anchor">Â§</a>Notes

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#object-safety" class="doc-anchor">Â§</a>Object Safety

`Read` uses `async in trait`, making it not object safe, preventing the use of `Box<dyn Read>`. To address this, weâ€™ve introduced [`ReadDyn`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html "trait opendal::raw::oio::ReadDyn") and its compatible type `Box<dyn ReadDyn>`.

`ReadDyn` uses `Box::pin()` to transform the returned future into a [`BoxedFuture`](https://opendal.apache.org/docs/rust/opendal/raw/type.BoxedFuture.html "type opendal::raw::BoxedFuture"), introducing an additional layer of indirection and an extra allocation. Ideally, `ReadDyn` should occur only once, at the outermost level of our API.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Read at the given offset with the given size.

## Provided Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#provided-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all" class="fn">read_all</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Read all data from the reader.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-Bytes" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for Bytes

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-Box%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.ReadDyn.html" class="trait" title="trait opendal::raw::oio::ReadDyn">ReadDyn</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T\>

#### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#note" class="doc-anchor">Â§</a>NOTE

Take care about the `deref_mut()` here. This makes sure that we are calling functions upon `&mut T` instead of `&mut Box<T>`. The later could result in infinite recursion.

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read-2" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#tymethod.read" class="fn">read</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#method.read_all" class="fn">read_all</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-Buffer" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-HttpBody" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/struct.HttpBody.html" class="struct" title="struct opendal::raw::HttpBody">HttpBody</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-FourWays%3CONE,+TWO,+THREE,+FOUR%3E" class="anchor">Â§</a>

### impl\<ONE, TWO, THREE, FOUR\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.FourWays.html" class="enum" title="enum opendal::raw::FourWays">FourWays</a>\<ONE, TWO, THREE, FOUR\>

where ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, THREE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, FOUR: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-TwoWays%3CONE,+TWO%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>\<ONE, TWO\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html#impl-Read-for-ThreeWays%3CONE,+TWO,+THREE%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>, THREE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Read.html" class="trait" title="trait opendal::raw::oio::Read">Read</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.ThreeWays.html" class="enum" title="enum opendal::raw::ThreeWays">ThreeWays</a>\<ONE, TWO, THREE\>
