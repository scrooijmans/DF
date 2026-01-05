# Trait PositionWrite Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/position_write.rs.html#46-59" class="src">Source</a>

``` rust
pub trait PositionWrite:
    Send
    + Sync
    + Unpin
    + 'static {
    // Required methods
    fn write_all_at(
        &self,
        offset: u64,
        buf: Buffer,
    ) -> impl Future<Output = Result<()>> + MaybeSend;
    fn close(&self) -> impl Future<Output = Result<Metadata>> + MaybeSend;
    fn abort(&self) -> impl Future<Output = Result<()>> + MaybeSend;
}
```

Expand description

PositionWrite is used to implement [`oio::Write`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html "trait opendal::raw::oio::Write") based on position write.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#services" class="doc-anchor">Â§</a>Services

Services like fs support position write.

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#architecture" class="doc-anchor">Â§</a>Architecture

The architecture after adopting [`PositionWrite`](https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html "trait opendal::raw::oio::PositionWrite"):

- Services impl `PositionWrite`
- `PositionWriter` impl `Write`
- Expose `PositionWriter` as `Accessor::Writer`

## <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#requirements" class="doc-anchor">Â§</a>Requirements

Services that implement `PositionWrite` must fulfill the following requirements:

- Writing data based on position: `offset`.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#tymethod.write_all_at" class="fn">write_all_at</a>( &self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, buf: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>, ) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

write_all_at is used to write the data to underlying storage at the specified offset.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#tymethod.close" class="fn">close</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

close is used to close the underlying file.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#tymethod.abort" class="fn">abort</a>(&self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

abort is used to abort the underlying abort.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html#implementors" class="anchor">Â§</a>
