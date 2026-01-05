# Struct CacheManager Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/cache/cache_manager.rs.html#132" class="src">Source</a>

``` rust
pub struct CacheManager { /* private fields */ }
```

Expand description

Manages various caches used in DataFusion.

Following DataFusion design principles, DataFusion provides default cache implementations, while also allowing users to provide their own custom cache implementations by implementing the relevant traits.

See [`CacheManagerConfig`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManagerConfig.html "struct datafusion::execution::cache::cache_manager::CacheManagerConfig") for configuration options.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#impl-CacheManager" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html" class="struct" title="struct datafusion::execution::cache::cache_manager::CacheManager">CacheManager</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#method.try_new" class="fn">try_new</a>( config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManagerConfig.html" class="struct" title="struct datafusion::execution::cache::cache_manager::CacheManagerConfig">CacheManagerConfig</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html" class="struct" title="struct datafusion::execution::cache::cache_manager::CacheManager">CacheManager</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#method.get_file_statistic_cache" class="fn">get_file_statistic_cache</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html" class="trait" title="trait datafusion::execution::cache::CacheAccessor">CacheAccessor</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>, Extra = <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>\>

Get the cache of listing files statistics.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#method.get_list_files_cache" class="fn">get_list_files_cache</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html" class="trait" title="trait datafusion::execution::cache::CacheAccessor">CacheAccessor</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>, Extra = <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>\>

Get the cache for storing the result of listing [`ObjectMeta`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html "struct object_store::ObjectMeta")s under the same path.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#method.get_file_metadata_cache" class="fn">get_file_metadata_cache</a>( &self, ) -\> <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/trait.FileMetadataCache.html" class="trait" title="trait datafusion::execution::cache::cache_manager::FileMetadataCache">FileMetadataCache</a>\<Extra = <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>\>\>

Get the file embedded metadata cache.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#method.get_metadata_cache_limit" class="fn">get_metadata_cache_limit</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Get the limit of the file embedded metadata cache.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#impl-Debug-for-CacheManager" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html" class="struct" title="struct datafusion::execution::cache::cache_manager::CacheManager">CacheManager</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html#blanket-implementations" class="anchor">§</a>
