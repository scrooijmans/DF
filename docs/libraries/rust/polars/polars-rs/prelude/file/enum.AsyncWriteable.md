# Enum AsyncWriteable Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/file.rs.html#255" class="src">Source</a>

``` rust
pub enum AsyncWriteable {
    Dyn(AsyncDynWriteable),
    Local(File),
    Cloud(BufWriter),
}
```

Available on **crate feature `polars-io`** only.

Expand description

Holds an async writeable file, abstracted over local files or cloud files.

This implements `DerefMut` to a trait object implementing [`tokio::io::AsyncWrite`](https://docs.rs/tokio/1.46.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html "trait tokio::io::async_write::AsyncWrite").

Note: It is important that you do not call `shutdown()` on the deref’ed `AsyncWrite` object. You should instead call the [`AsyncWriteable::close`](https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#method.close "method polars::prelude::file::AsyncWriteable::close") at the end.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#variant.Dyn" class="anchor">§</a>

### Dyn(AsyncDynWriteable)

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#variant.Local" class="anchor">§</a>

### Local(<a href="https://docs.rs/tokio/1.46.1/x86_64-unknown-linux-gnu/tokio/fs/file/struct.File.html" class="struct" title="struct tokio::fs::file::File">File</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#variant.Cloud" class="anchor">§</a>

### Cloud(<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/buffered/struct.BufWriter.html" class="struct" title="struct object_store::buffered::BufWriter">BufWriter</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#impl-AsyncWriteable" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#method.try_new" class="fn">try_new</a>( addr: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'\_\>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#method.sync_on_close" class="fn">sync_on_close</a>( &mut self, sync_on_close: <a href="https://docs.rs/polars/latest/polars/prelude/sync_on_close/enum.SyncOnCloseType.html" class="enum" title="enum polars::prelude::sync_on_close::SyncOnCloseType">SyncOnCloseType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#method.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#impl-Deref-for-AsyncWriteable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = dyn <a href="https://docs.rs/tokio/1.46.1/x86_64-unknown-linux-gnu/tokio/io/async_write/trait.AsyncWrite.html" class="trait" title="trait tokio::io::async_write::AsyncWrite">AsyncWrite</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html" class="trait" title="trait core::marker::Unpin">Unpin</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#impl-DerefMut-for-AsyncWriteable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html" class="trait" title="trait core::ops::deref::DerefMut">DerefMut</a> for <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#method.deref_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut" class="fn">deref_mut</a>(&mut self) -\> &mut \<<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Mutably dereferences the value.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html#blanket-implementations" class="anchor">§</a>
