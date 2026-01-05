# Trait FileMetadataCache Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/cache/cache_manager.rs.html#80-81" class="src">Source</a>

``` rust
pub trait FileMetadataCache: CacheAccessor<ObjectMeta, Arc<dyn FileMetadata>, Extra = ObjectMeta> {
    // Required methods
    fn cache_limit(&self) -> usize;
    fn update_cache_limit(&self, limit: usize);
    fn list_entries(&self) -> HashMap<Path, FileMetadataCacheEntry>;
}
```

Expand description

Cache for file-embedded metadata.

This cache stores per-file metadata in the form of [`FileMetadata`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadata.html "trait datafusion::execution::cache::cache_manager::FileMetadata"),

For example, the built in [`ListingTable`](https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTable.html) uses this cache to avoid parsing Parquet footers multiple times for the same file.

DataFusion provides a default implementation, [`DefaultFilesMetadataCache`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFilesMetadataCache.html "struct datafusion::execution::cache::cache_unit::DefaultFilesMetadataCache"), and users can also provide their own implementations to implement custom caching strategies.

See [`crate::runtime_env::RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv") for more details.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#tymethod.cache_limit" class="fn">cache_limit</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the cache’s memory limit in bytes.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#tymethod.update_cache_limit" class="fn">update_cache_limit</a>(&self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)

Updates the cache with a new memory limit in bytes.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#tymethod.list_entries" class="fn">list_entries</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.FileMetadataCacheEntry.html" class="struct" title="struct datafusion::execution::cache::cache_manager::FileMetadataCacheEntry">FileMetadataCacheEntry</a>\>

Retrieves the information about the entries currently cached.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#impl-Debug-for-dyn+FileMetadataCache" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html" class="trait" title="trait datafusion::execution::cache::cache_manager::FileMetadataCache">FileMetadataCache</a>\<Extra = <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html#impl-FileMetadataCache-for-DefaultFilesMetadataCache" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html" class="trait" title="trait datafusion::execution::cache::cache_manager::FileMetadataCache">FileMetadataCache</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFilesMetadataCache.html" class="struct" title="struct datafusion::execution::cache::cache_unit::DefaultFilesMetadataCache">DefaultFilesMetadataCache</a>
