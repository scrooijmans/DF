# Function partitioned_file_groups Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/test/mod.rs.html#104-190" class="src">Source</a>

``` rust
pub fn partitioned_file_groups(
    path: &str,
    filename: &str,
    partitions: usize,
    file_format: Arc<dyn FileFormat>,
    file_compression_type: FileCompressionType,
    work_dir: &Path,
) -> Result<Vec<FileGroup>>
```

Available on **non-WebAssembly** only.

Expand description

Returns file groups [`Vec<FileGroup>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec") for scanning `partitions` of `filename`
