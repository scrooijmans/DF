# Struct IpcReaderAsync Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ipc/ipc_reader_async.rs.html#23" class="src">Source</a>

``` rust
pub struct IpcReaderAsync { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

An Arrow IPC reader implemented on top of PolarsObjectStore.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#impl-IpcReaderAsync" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html" class="struct" title="struct polars::prelude::IpcReaderAsync">IpcReaderAsync</a>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#method.from_uri" class="fn">from_uri</a>( uri: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html" class="struct" title="struct polars::prelude::IpcReaderAsync">IpcReaderAsync</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#method.metadata" class="fn">metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/io/ipc/read/file/struct.FileMetadata.html" class="struct" title="struct polars_arrow::io::ipc::read::file::FileMetadata">FileMetadata</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#method.data" class="fn">data</a>( &self, metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/io/ipc/read/file/struct.FileMetadata.html" class="struct" title="struct polars_arrow::io::ipc::read::file::FileMetadata">FileMetadata</a>\>, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReadOptions.html" class="struct" title="struct polars::prelude::IpcReadOptions">IpcReadOptions</a>, verbose: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#method.count_rows" class="fn">count_rows</a>( &self, \_metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/io/ipc/read/file/struct.FileMetadata.html" class="struct" title="struct polars_arrow::io::ipc::read::file::FileMetadata">FileMetadata</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReaderAsync.html#blanket-implementations" class="anchor">§</a>
