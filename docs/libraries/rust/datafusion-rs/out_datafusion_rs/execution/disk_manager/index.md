# Module disk_manager Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/lib.rs.html#31" class="src">Source</a>

Expand description

[`DiskManager`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager"): Manages files generated during query execution

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManager.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManager">DiskManager</a>  
Manages files generated during query execution, e.g. spill files generated while processing dataset larger than available memory.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>  
Builder pattern for the [DiskManager](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager") structure

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.RefCountedTempFile.html" class="struct" title="struct datafusion::execution::disk_manager::RefCountedTempFile">RefCountedTempFile</a>  
A wrapper around a [`NamedTempFile`](https://docs.rs/tempfile/3.21.0/x86_64-unknown-linux-gnu/tempfile/file/struct.NamedTempFile.html "struct tempfile::file::NamedTempFile") that also contains a reference to its parent temporary directory.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>Deprecated

Configuration for temporary disk access

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerMode.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerMode">DiskManagerMode</a>
