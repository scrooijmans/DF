# Trait DynWriteable Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/utils/file.rs.html#17" class="src">Source</a>

``` rust
pub trait DynWriteable: Write + Send {
    // Required methods
    fn as_dyn_write(&self) -> &(dyn Write + Send + 'static);
    fn as_mut_dyn_write(&mut self) -> &mut (dyn Write + Send + 'static);
    fn close(self: Box<Self>) -> Result<(), Error>;
    fn sync_on_close(
        &mut self,
        sync_on_close: SyncOnCloseType,
    ) -> Result<(), Error>;
}
```

Available on **crate feature `polars-io`** only.

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.as_dyn_write" class="fn">as_dyn_write</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.as_mut_dyn_write" class="fn">as_mut_dyn_write</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static)

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.close" class="fn">close</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<Self\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.sync_on_close" class="fn">sync_on_close</a>(&mut self, sync_on_close: <a href="https://docs.rs/polars/latest/polars/prelude/sync_on_close/enum.SyncOnCloseType.html" class="enum" title="enum polars::prelude::sync_on_close::SyncOnCloseType">SyncOnCloseType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

## Implementations on Foreign Types<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#foreign-impls" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#impl-DynWriteable-for-ClosableFile" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html" class="trait" title="trait polars::prelude::file::DynWriteable">DynWriteable</a> for <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/file/struct.ClosableFile.html" class="struct" title="struct polars_utils::file::ClosableFile">ClosableFile</a>

<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#method.as_dyn_write" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.as_dyn_write" class="fn">as_dyn_write</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static)

<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#method.as_mut_dyn_write" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.as_mut_dyn_write" class="fn">as_mut_dyn_write</a>(&mut self) -\> &mut (dyn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'static)

<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#method.close" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.close" class="fn">close</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/file/struct.ClosableFile.html" class="struct" title="struct polars_utils::file::ClosableFile">ClosableFile</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#method.sync_on_close" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#tymethod.sync_on_close" class="fn">sync_on_close</a>(&mut self, sync_on_close: <a href="https://docs.rs/polars/latest/polars/prelude/sync_on_close/enum.SyncOnCloseType.html" class="enum" title="enum polars::prelude::sync_on_close::SyncOnCloseType">SyncOnCloseType</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/file/trait.DynWriteable.html#implementors" class="anchor">§</a>
