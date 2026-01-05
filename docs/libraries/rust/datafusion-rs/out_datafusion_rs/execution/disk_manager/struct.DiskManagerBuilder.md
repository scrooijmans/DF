# Struct DiskManagerBuilder Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/disk_manager.rs.html#37" class="src">Source</a>

``` rust
pub struct DiskManagerBuilder { /* private fields */ }
```

Expand description

Builder pattern for the [DiskManager](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager") structure

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#impl-DiskManagerBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.set_mode" class="fn">set_mode</a>(&mut self, mode: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerMode.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerMode">DiskManagerMode</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.with_mode" class="fn">with_mode</a>(self, mode: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerMode.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerMode">DiskManagerMode</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.set_max_temp_directory_size" class="fn">set_max_temp_directory_size</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.with_max_temp_directory_size" class="fn">with_max_temp_directory_size</a>(self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Create a DiskManager given the builder

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#impl-Clone-for-DiskManagerBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#impl-Debug-for-DiskManagerBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#impl-Default-for-DiskManagerBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html#blanket-implementations" class="anchor">§</a>
