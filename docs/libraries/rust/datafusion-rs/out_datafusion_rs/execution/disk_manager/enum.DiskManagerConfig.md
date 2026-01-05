# Enum DiskManagerConfig Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/disk_manager.rs.html#125" class="src">Source</a>

``` rust
pub enum DiskManagerConfig {
    Existing(Arc<DiskManager>),
    NewOs,
    NewSpecified(Vec<PathBuf>),
    Disabled,
}
```

👎Deprecated since 48.0.0: Use DiskManagerBuilder instead

Expand description

Configuration for temporary disk access

## Variants<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#variants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#variant.Existing" class="anchor">§</a>

### Existing(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>\>)

👎Deprecated since 48.0.0: Use DiskManagerBuilder instead

Use the provided [DiskManager](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager") instance

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#variant.NewOs" class="anchor">§</a>

### NewOs

👎Deprecated since 48.0.0: Use DiskManagerBuilder instead

Create a new [DiskManager](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager") that creates temporary files within a temporary directory chosen by the OS

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#variant.NewSpecified" class="anchor">§</a>

### NewSpecified(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>)

👎Deprecated since 48.0.0: Use DiskManagerBuilder instead

Create a new [DiskManager](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager") that creates temporary files within the specified directories

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#variant.Disabled" class="anchor">§</a>

### Disabled

👎Deprecated since 48.0.0: Use DiskManagerBuilder instead

Disable disk manager, attempts to create temporary files will error

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#impl-DiskManagerConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

Create temporary files in a temporary directory chosen by the OS

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#method.new_existing" class="fn">new_existing</a>(existing: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager">DiskManager</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

Create temporary files using the provided disk manager

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#method.new_specified" class="fn">new_specified</a>(paths: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

Create temporary files in the specified directories

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#impl-Clone-for-DiskManagerConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#impl-Debug-for-DiskManagerConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#impl-Default-for-DiskManagerConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html#blanket-implementations" class="anchor">§</a>
