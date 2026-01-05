# Struct OutputFile Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/file_io.rs.html#406-412" class="src">Source</a>

``` rust
pub struct OutputFile { /* private fields */ }
```

Expand description

Output file is used for writing to files..

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#impl-OutputFile" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.location" class="fn">location</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Relative path to root uri.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.exists" class="fn">exists</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Checks if file exists.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.delete" class="fn">delete</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Deletes file.

If the file does not exist, it will not return error.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.to_input_file" class="fn">to_input_file</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html" class="struct" title="struct iceberg::io::InputFile">InputFile</a>

Converts into [`InputFile`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html "struct iceberg::io::InputFile").

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.write" class="fn">write</a>(&self, bs: <a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Create a new output file with given bytes.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#notes" class="doc-anchor">§</a>Notes

Calling `write` will overwrite the file if it exists. For continuous writing, use [`Self::writer`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.writer "method iceberg::io::OutputFile::writer").

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.writer" class="fn">writer</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/trait.FileWrite.html" class="trait" title="trait iceberg::io::FileWrite">FileWrite</a>\>\>

Creates output file for continuous writing.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#notes-1" class="doc-anchor">§</a>Notes

For one-time writing, use [`Self::write`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.write "method iceberg::io::OutputFile::write") instead.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#impl-Debug-for-OutputFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html#blanket-implementations" class="anchor">§</a>
