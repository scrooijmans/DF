# Struct BlockingCloudWriter Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/cloud/adaptors.rs.html#26" class="src">Source</a>

``` rust
pub struct BlockingCloudWriter { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Adaptor which wraps the interface of [ObjectStore::BufWriter](https://docs.rs/object_store/latest/object_store/buffered/struct.BufWriter.html) exposing a synchronous interface which implements `std::io::Write`.

This allows it to be used in sync code which would otherwise write to a simple File or byte stream, such as with `polars::prelude::CsvWriter`.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#impl-BlockingCloudWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.new_with_object_store" class="fn">new_with_object_store</a>( object_store: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, path: <a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Construct a new BlockingCloudWriter, re-using the given `object_store`

Creates a new (current-thread) Tokio runtime which bridges the sync writing process with the async ObjectStore multipart uploading. TODO: Naming?

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.new" class="fn">new</a>( uri: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Constructs a new BlockingCloudWriter from a path and an optional set of CloudOptions.

Wrapper around `BlockingCloudWriter::new_with_object_store` that is useful if you only have a single write task. TODO: Naming?

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.try_into_inner" class="fn">try_into_inner</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/buffered/struct.BufWriter.html" class="struct" title="struct object_store::buffered::BufWriter">BufWriter</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Returns the underlying [`object_store::buffered::BufWriter`](https://docs.rs/object_store/0.12.2/x86_64-unknown-linux-gnu/object_store/buffered/struct.BufWriter.html "struct object_store::buffered::BufWriter")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.close" class="fn">close</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Closes the writer, or returns the existing error if it exists. After this function is called the writer is guaranteed to be in an error state.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#impl-Drop-for-BlockingCloudWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#impl-Write-for-BlockingCloudWriter" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.write" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write" class="fn">write</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a buffer into this writer, returning how many bytes were written. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.flush" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush" class="fn">flush</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Flushes this output stream, ensuring that all intermediately buffered contents reach their destination. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.flush)

1.36.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1758" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored" class="fn">write_vectored</a>(&mut self, bufs: &\[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Like [`write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#tymethod.write "method std::io::Write::write"), except that it writes from a slice of buffers. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.is_write_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored" class="fn">is_write_vectored</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

🔬This is a nightly-only experimental API. (`can_vector`)

Determines if this `Write`r has an efficient [`write_vectored`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_vectored "method std::io::Write::write_vectored") implementation. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.is_write_vectored)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1835" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.write_all" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all" class="fn">write_all</a>(&mut self, buf: &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Attempts to write an entire buffer into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.write_all_vectored" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored" class="fn">write_all_vectored</a>(&mut self, bufs: &mut \[<a href="https://doc.rust-lang.org/nightly/std/io/struct.IoSlice.html" class="struct" title="struct std::io::IoSlice">IoSlice</a>\<'\_\>\]) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

🔬This is a nightly-only experimental API. (`write_all_vectored`)

Attempts to write multiple buffers into this writer. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_all_vectored)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1950" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.write_fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt" class="fn">write_fmt</a>(&mut self, args: <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Arguments.html" class="struct" title="struct core::fmt::Arguments">Arguments</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

Writes a formatted string into this writer, returning any error encountered. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.write_fmt)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/std/io/mod.rs.html#1980-1982" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.by_ref" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref" class="fn">by_ref</a>(&mut self) -\> &mut Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Creates a “by reference” adapter for this instance of `Write`. [Read more](https://doc.rust-lang.org/nightly/std/io/trait.Write.html#method.by_ref)

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#impl-WriteClose-for-BlockingCloudWriter" class="anchor">§</a>

### impl <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/file/trait.WriteClose.html" class="trait" title="trait polars_utils::file::WriteClose">WriteClose</a> for <a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>

<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#method.close-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars-utils/0.51.0/x86_64-unknown-linux-gnu/polars_utils/file/trait.WriteClose.html#method.close" class="fn">close</a>(self: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html" class="struct" title="struct polars::prelude::cloud::BlockingCloudWriter">BlockingCloudWriter</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html" class="struct" title="struct std::io::error::Error">Error</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.BlockingCloudWriter.html#blanket-implementations" class="anchor">§</a>
