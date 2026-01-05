# Struct RefCountedTempFile Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/disk_manager.rs.html#315" class="src">Source</a>

``` rust
pub struct RefCountedTempFile { /* private fields */ }
```

Expand description

A wrapper around a [`NamedTempFile`](https://docs.rs/tempfile/3.21.0/x86_64-unknown-linux-gnu/tempfile/file/struct.NamedTempFile.html "struct tempfile::file::NamedTempFile") that also contains a reference to its parent temporary directory.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#note" class="doc-anchor">§</a>Note

After any modification to the underlying file (e.g., writing data to it), the caller must invoke [`Self::update_disk_usage`](https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#method.update_disk_usage "method datafusion::execution::disk_manager::RefCountedTempFile::update_disk_usage") to update the global disk usage counter. This ensures the disk manager can properly enforce usage limits configured by [`DiskManager::with_max_temp_directory_size`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.with_max_temp_directory_size "method datafusion::execution::DiskManager::with_max_temp_directory_size").

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#impl-RefCountedTempFile" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html" class="struct" title="struct datafusion::execution::disk_manager::RefCountedTempFile">RefCountedTempFile</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#method.path" class="fn">path</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/path/struct.Path.html" class="struct" title="struct std::path::Path">Path</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#method.inner" class="fn">inner</a>(&self) -\> &<a href="https://docs.rs/tempfile/3.21.0/x86_64-unknown-linux-gnu/tempfile/file/struct.NamedTempFile.html" class="struct" title="struct tempfile::file::NamedTempFile">NamedTempFile</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#method.update_disk_usage" class="fn">update_disk_usage</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Updates the global disk usage counter after modifications to the underlying file.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#errors" class="doc-anchor">§</a>Errors

- Returns an error if the global disk usage exceeds the configured limit.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#method.current_disk_usage" class="fn">current_disk_usage</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#impl-Debug-for-RefCountedTempFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html" class="struct" title="struct datafusion::execution::disk_manager::RefCountedTempFile">RefCountedTempFile</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#impl-Drop-for-RefCountedTempFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html" class="struct" title="struct datafusion::execution::disk_manager::RefCountedTempFile">RefCountedTempFile</a>

When the temporary file is dropped, subtract its disk usage from the disk manager’s total

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html#blanket-implementations" class="anchor">§</a>
