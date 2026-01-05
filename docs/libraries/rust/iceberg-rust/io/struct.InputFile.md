# Struct InputFile Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/file_io.rs.html#315-321" class="src">Source</a>

``` rust
pub struct InputFile { /* private fields */ }
```

Expand description

Input file is used for reading from files.

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#impl-InputFile" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html" class="struct" title="struct iceberg::io::InputFile">InputFile</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.location" class="fn">location</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Absolute path to root uri.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.exists" class="fn">exists</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Check if file exists.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.metadata" class="fn">metadata</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileMetadata.html" class="struct" title="struct iceberg::io::FileMetadata">FileMetadata</a>\>

Fetch and returns metadata of file.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.read" class="fn">read</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>\>

Read and returns whole content of file.

For continuous reading, use [`Self::reader`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.reader "method iceberg::io::InputFile::reader") instead.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.reader" class="fn">reader</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html" class="trait" title="trait iceberg::io::FileRead">FileRead</a> + use\<\>\>

Creates [`FileRead`](https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileRead.html "trait iceberg::io::FileRead") for continuous reading.

For one-time reading, use [`Self::read`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.read "method iceberg::io::InputFile::read") instead.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#impl-Debug-for-InputFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html" class="struct" title="struct iceberg::io::InputFile">InputFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html#blanket-implementations" class="anchor">§</a>
