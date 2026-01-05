# Struct FileIO Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/io/file_io.rs.html#48-52" class="src">Source</a>

``` rust
pub struct FileIO { /* private fields */ }
```

Expand description

FileIO implementation, used to manipulate files in underlying storage.

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#note" class="doc-anchor">§</a>Note

All path passed to `FileIO` must be absolute path starting with scheme string used to construct `FileIO`. For example, if you construct `FileIO` with `s3a` scheme, then all path passed to `FileIO` must start with `s3a://`.

Supported storages:

| Storage | Feature Flag | Expected Path Format | Schemes |
|----|----|----|----|
| Local file system | `storage-fs` | `file` | `file://path/to/file` |
| Memory | `storage-memory` | `memory` | `memory://path/to/file` |
| S3 | `storage-s3` | `s3`, `s3a` | `s3://<bucket>/path/to/file` |
| GCS | `storage-gcs` | `gs`, `gcs` | `gs://<bucket>/path/to/file` |
| OSS | `storage-oss` | `oss` | `oss://<bucket>/path/to/file` |
| Azure Datalake | `storage-azdls` | `abfs`, `abfss`, `wasb`, `wasbs` | `abfs://<filesystem>@<account>.dfs.core.windows.net/path/to/file` or `wasb://<container>@<account>.blob.core.windows.net/path/to/file` |

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#impl-FileIO" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.into_builder" class="fn">into_builder</a>(self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html" class="struct" title="struct iceberg::io::FileIOBuilder">FileIOBuilder</a>

Convert FileIO into [`FileIOBuilder`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html "struct iceberg::io::FileIOBuilder") which used to build this FileIO.

This function is useful when you want serialize and deserialize FileIO across distributed systems.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.from_path" class="fn">from_path</a>(path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIOBuilder.html" class="struct" title="struct iceberg::io::FileIOBuilder">FileIOBuilder</a>\>

Try to infer file io scheme from path. See [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO") for supported schemes.

- If it’s a valid url, for example `s3://bucket/a`, url scheme will be used, and the rest of the url will be ignored.
- If it’s not a valid url, will try to detect if it’s a file path.

Otherwise will return parsing error.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.delete" class="fn">delete</a>(&self, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Deletes file.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#arguments" class="doc-anchor">§</a>Arguments

- path: It should be *absolute* path starting with scheme string used to construct [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.remove_all" class="fn">remove_all</a>(&self, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

👎Deprecated since 0.4.0: use remove_dir_all instead

Remove the path and all nested dirs and files recursively.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#arguments-1" class="doc-anchor">§</a>Arguments

- path: It should be *absolute* path starting with scheme string used to construct [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.remove_dir_all" class="fn">remove_dir_all</a>(&self, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>\>

Remove the path and all nested dirs and files recursively.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#arguments-2" class="doc-anchor">§</a>Arguments

- path: It should be *absolute* path starting with scheme string used to construct [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#behavior" class="doc-anchor">§</a>Behavior

- If the path is a file or not exist, this function will be no-op.
- If the path is a empty directory, this function will remove the directory itself.
- If the path is a non-empty directory, this function will remove the directory and all nested files and directories.

#### pub async fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.exists" class="fn">exists</a>(&self, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>

Check file exists.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#arguments-3" class="doc-anchor">§</a>Arguments

- path: It should be *absolute* path starting with scheme string used to construct [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.new_input" class="fn">new_input</a>(&self, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.InputFile.html" class="struct" title="struct iceberg::io::InputFile">InputFile</a>\>

Creates input file.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#arguments-4" class="doc-anchor">§</a>Arguments

- path: It should be *absolute* path starting with scheme string used to construct [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.new_output" class="fn">new_output</a>(&self, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/type.Result.html" class="type" title="type iceberg::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.OutputFile.html" class="struct" title="struct iceberg::io::OutputFile">OutputFile</a>\>

Creates output file.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#arguments-5" class="doc-anchor">§</a>Arguments

- path: It should be *absolute* path starting with scheme string used to construct [`FileIO`](https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html "struct iceberg::io::FileIO").

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#impl-Clone-for-FileIO" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#impl-Debug-for-FileIO" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html" class="struct" title="struct iceberg::io::FileIO">FileIO</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/io/struct.FileIO.html#blanket-implementations" class="anchor">§</a>
