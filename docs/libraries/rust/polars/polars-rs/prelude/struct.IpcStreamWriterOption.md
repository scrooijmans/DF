# Struct IpcStreamWriterOption Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ipc/ipc_stream.rs.html#291" class="src">Source</a>

``` rust
pub struct IpcStreamWriterOption { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#impl-IpcStreamWriterOption" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html" class="struct" title="struct polars::prelude::IpcStreamWriterOption">IpcStreamWriterOption</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html" class="struct" title="struct polars::prelude::IpcStreamWriterOption">IpcStreamWriterOption</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#method.with_compression" class="fn">with_compression</a>( self, compression: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.IpcCompression.html" class="enum" title="enum polars::prelude::IpcCompression">IpcCompression</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html" class="struct" title="struct polars::prelude::IpcStreamWriterOption">IpcStreamWriterOption</a>

Set the compression used. Defaults to None.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#method.with_extension" class="fn">with_extension</a>(self, extension: <a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html" class="struct" title="struct polars::prelude::IpcStreamWriterOption">IpcStreamWriterOption</a>

Set the extension. Defaults to “.ipc”.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#impl-Default-for-IpcStreamWriterOption" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html" class="struct" title="struct polars::prelude::IpcStreamWriterOption">IpcStreamWriterOption</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html" class="struct" title="struct polars::prelude::IpcStreamWriterOption">IpcStreamWriterOption</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriterOption.html#blanket-implementations" class="anchor">§</a>
