# Trait AppendWrite Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/append_write.rs.html#39-52" class="src">Source</a>

``` rust
pub trait AppendWrite:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required methods
    fn offset(&self) -> impl Future<Output = Result<u64>> + MaybeSend;
    fn append(
        &self,
        offset: u64,
        size: u64,
        body: Buffer,
    ) -> impl Future<Output = Result<Metadata>> + MaybeSend;
}
```

Expand description

AppendWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on append object. By implementing AppendWrite, services donâ€™t need to care about the details of buffering and uploading parts.

The layout after adopting [`AppendWrite`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html "trait opendal::raw::oio::AppendWrite"):

- Services impl `AppendWrite`
- `AppendWriter` impl `Write`
- Expose `AppendWriter` as `Accessor::Writer`

### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html#requirements" class="doc-anchor">Â§</a>Requirements

Services that implement `AppendWrite` must fulfill the following requirements:

- Must be a http service that could accept `AsyncBody`.
- Provide a way to get the current offset of the append object.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html#tymethod.offset" class="fn">offset</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Get the current offset of the append object.

Returns `0` if the object is not exist.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html#tymethod.append" class="fn">append</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, body: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Append the data to the end of this object.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html#implementors" class="anchor">Â§</a>
