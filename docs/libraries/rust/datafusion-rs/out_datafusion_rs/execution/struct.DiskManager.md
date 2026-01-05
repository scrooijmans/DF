# Struct DiskManager Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/disk_manager.rs.html#169" class="src">Source</a>

``` rust
pub struct DiskManager { /* private fields */ }
```

Expand description

Manages files generated during query execution, e.g. spill files generated while processing dataset larger than available memory.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#impl-DiskManager" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.builder" class="fn">builder</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

Creates a builder for [DiskManager](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.try_new" class="fn">try_new</a>( config: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

👎Deprecated since 48.0.0: Use DiskManager::builder() instead

Create a DiskManager given the configuration

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.set_max_temp_directory_size" class="fn">set_max_temp_directory_size</a>( &mut self, max_temp_directory_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.set_arc_max_temp_directory_size" class="fn">set_arc_max_temp_directory_size</a>( this: &mut <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>\>, max_temp_directory_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.with_max_temp_directory_size" class="fn">with_max_temp_directory_size</a>( self, max_temp_directory_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.used_disk_space" class="fn">used_disk_space</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.tmp_files_enabled" class="fn">tmp_files_enabled</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Return true if this disk manager supports creating temporary files. If this returns false, any call to `create_tmp_file` will error.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.create_tmp_file" class="fn">create_tmp_file</a>( self: &<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>\>, request_description: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html" class="struct" title="struct datafusion::execution::disk_manager::RefCountedTempFile">RefCountedTempFile</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Return a temporary file from a randomized choice in the configured locations

If the file can not be created for some reason, returns an error message referencing the request description

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#impl-Debug-for-DiskManager" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html#blanket-implementations" class="anchor">§</a>
