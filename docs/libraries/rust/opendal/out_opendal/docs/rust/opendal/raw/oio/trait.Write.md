# Trait Write Copy item path

<a href="https://opendal.apache.org/docs/rust/src/opendal/raw/oio/write/api.rs.html#28-42" class="src">Source</a>

``` rust
pub trait Write:
    Unpin
    + Send
    + Sync {
    // Required methods
    fn write(
        &mut self,
        bs: Buffer,
    ) -> impl Future<Output = Result<()>> + MaybeSend;
    fn close(&mut self) -> impl Future<Output = Result<Metadata>> + MaybeSend;
    fn abort(&mut self) -> impl Future<Output = Result<()>> + MaybeSend;
}
```

Expand description

Write is the trait that OpenDAL returns to callers.

## Required Methods<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#required-methods" class="anchor">Â§</a>

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Write given bytes into writer.

##### <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#behavior" class="doc-anchor">Â§</a>Behavior

- `Ok(())` means all bytes has been written successfully.
- `Err(err)` means error happens and no bytes has been written.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Close the writer and make sure all data has been flushed.

#### fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> impl <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>\> + <a href="https://opendal.apache.org/docs/rust/opendal/raw/trait.MaybeSend.html" class="trait" title="trait opendal::raw::MaybeSend">MaybeSend</a>

Abort the pending writer.

## Dyn Compatibility<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#dyn-compatibility" class="anchor">Â§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementations on Foreign Types<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#foreign-impls" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-()" class="anchor">Â§</a>

### impl <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#method.write" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, \_: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#method.close" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#method.abort" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-Box%3CT%3E" class="anchor">Â§</a>

### impl\<T: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.WriteDyn.html" class="trait" title="trait opendal::raw::oio::WriteDyn">WriteDyn</a> + ?<a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<T\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#method.write-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, bs: <a href="https://opendal.apache.org/docs/rust/opendal/struct.Buffer.html" class="struct" title="struct opendal::Buffer">Buffer</a>) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#method.close-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.close" class="fn">close</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://opendal.apache.org/docs/rust/opendal/struct.Metadata.html" class="struct" title="struct opendal::Metadata">Metadata</a>\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#method.abort-1" class="anchor">Â§</a>

#### async fn <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#tymethod.abort" class="fn">abort</a>(&mut self) -\> <a href="https://opendal.apache.org/docs/rust/opendal/type.Result.html" class="type" title="type opendal::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

## Implementors<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#implementors" class="anchor">Â§</a>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-TwoWays%3CONE,+TWO%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.TwoWays.html" class="enum" title="enum opendal::raw::TwoWays">TwoWays</a>\<ONE, TWO\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-ThreeWays%3CONE,+TWO,+THREE%3E" class="anchor">Â§</a>

### impl\<ONE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>, TWO: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>, THREE: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/enum.ThreeWays.html" class="enum" title="enum opendal::raw::ThreeWays">ThreeWays</a>\<ONE, TWO, THREE\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-AppendWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.AppendWriter.html" class="struct" title="struct opendal::raw::oio::AppendWriter">AppendWriter</a>\<W\>

where W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.AppendWrite.html" class="trait" title="trait opendal::raw::oio::AppendWrite">AppendWrite</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-BlockWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.BlockWriter.html" class="struct" title="struct opendal::raw::oio::BlockWriter">BlockWriter</a>\<W\>

where W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.BlockWrite.html" class="trait" title="trait opendal::raw::oio::BlockWrite">BlockWrite</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-MultipartWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.MultipartWriter.html" class="struct" title="struct opendal::raw::oio::MultipartWriter">MultipartWriter</a>\<W\>

where W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.MultipartWrite.html" class="trait" title="trait opendal::raw::oio::MultipartWrite">MultipartWrite</a>,

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-OneShotWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.OneShotWrite.html" class="trait" title="trait opendal::raw::oio::OneShotWrite">OneShotWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.OneShotWriter.html" class="struct" title="struct opendal::raw::oio::OneShotWriter">OneShotWriter</a>\<W\>

<a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html#impl-Write-for-PositionWriter%3CW%3E" class="anchor">Â§</a>

### impl\<W: <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.PositionWrite.html" class="trait" title="trait opendal::raw::oio::PositionWrite">PositionWrite</a>\> <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/trait.Write.html" class="trait" title="trait opendal::raw::oio::Write">Write</a> for <a href="https://opendal.apache.org/docs/rust/opendal/raw/oio/struct.PositionWriter.html" class="struct" title="struct opendal::raw::oio::PositionWriter">PositionWriter</a>\<W\>
