# Function split_files Copy item path

<a href="https://docs.rs/datafusion-catalog-listing/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog_listing/helpers.rs.html#126-129" class="src">Source</a>

``` rust
pub fn split_files(
    partitioned_files: Vec<PartitionedFile>,
    n: usize,
) -> Vec<Vec<PartitionedFile>>
```

👎Deprecated since 47.0.0: use `FileGroup::split_files` instead

Expand description

Partition the list of files into `n` groups
