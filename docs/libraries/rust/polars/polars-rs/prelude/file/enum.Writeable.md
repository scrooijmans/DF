# Enum Writeable Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/file.rs.html#47" class="src">Source</a>

``` rust
pub enum Writeable {
    Dyn(Box<dyn DynWriteable>),
    Local(File),
    Cloud(BlockingCloudWriter),
}
```

Available on **crate feature `polars-io`** only.

Expand description

Holds a non-async writeable file, abstracted over local files or cloud files.

This implements `DerefMut` to a trait object implementing [`std::io::Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write").

Also see: `Writeable::try_into_async_writeable` and `AsyncWriteable`.

## Variants<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#variant.Dyn" class="anchor">§</a>

### Dyn(<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html" class="trait" title="trait polars::prelude::file::DynWriteable">DynWriteable</a>\>)

An abstract implementation for writable.

This is used to implement writing to in-memory and arbitrary file descriptors.

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#variant.Local" class="anchor">§</a>

### Local(<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>)

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#variant.Cloud" class="anchor">§</a>

### Cloud(<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>)

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#impl-Writeable" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html" class="enum" title="enum polars::prelude::file::Writeable">Writeable</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.try_new" class="fn">try_new</a>( addr: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPathRef.html" class="enum" title="enum polars::prelude::PlPathRef">PlPathRef</a>\<'\_\>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html" class="enum" title="enum polars::prelude::file::Writeable">Writeable</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.try_into_async_writeable" class="fn">try_into_async_writeable</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.AsyncWriteable.html" class="enum" title="enum polars::prelude::file::AsyncWriteable">AsyncWriteable</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

This returns `Result<>` - if a write was performed before calling this, `CloudWriter` can be in an Err(\_) state.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.sync_on_close" class="fn">sync_on_close</a>( &mut self, sync_on_close: <a href="https://docs.rs/polars/latest/polars/prelude/sync_on_close/enum.SyncOnCloseType.html" class="enum" title="enum polars::prelude::sync_on_close::SyncOnCloseType">SyncOnCloseType</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.close" class="fn">close</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#impl-Deref-for-Writeable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a> for <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html" class="enum" title="enum polars::prelude::file::Writeable">Writeable</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#associatedtype.Target" class="anchor">§</a>

#### type <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype">Target</a> = dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>

The resulting type after dereferencing.

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.deref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#tymethod.deref" class="fn">deref</a>(&self) -\> &\<<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html" class="enum" title="enum polars::prelude::file::Writeable">Writeable</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Dereferences the value.

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#impl-DerefMut-for-Writeable" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html" class="trait" title="trait core::ops::deref::DerefMut">DerefMut</a> for <a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html" class="enum" title="enum polars::prelude::file::Writeable">Writeable</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#method.deref_mut" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html#tymethod.deref_mut" class="fn">deref_mut</a>(&mut self) -\> &mut \<<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html" class="enum" title="enum polars::prelude::file::Writeable">Writeable</a> as <a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html" class="trait" title="trait core::ops::deref::Deref">Deref</a>\>::<a href="https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html#associatedtype.Target" class="associatedtype" title="type core::ops::deref::Deref::Target">Target</a>

Mutably dereferences the value.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/file/enum.Writeable.html#blanket-implementations" class="anchor">§</a>
