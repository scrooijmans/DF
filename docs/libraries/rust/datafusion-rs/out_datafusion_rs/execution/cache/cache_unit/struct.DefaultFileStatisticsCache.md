# Struct DefaultFileStatisticsCache Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/cache/cache_unit.rs.html#41" class="src">Source</a>

``` rust
pub struct DefaultFileStatisticsCache { /* private fields */ }
```

Expand description

Default implementation of [`FileStatisticsCache`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/type.FileStatisticsCache.html "type datafusion::execution::cache::cache_manager::FileStatisticsCache")

Stores collected statistics for files

Cache is invalided when file size or last modification has changed

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#impl-CacheAccessor%3CPath,+Arc%3CStatistics%3E%3E-for-DefaultFileStatisticsCache" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html" class="trait" title="trait datafusion::execution::cache::CacheAccessor">CacheAccessor</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html" class="struct" title="struct datafusion::execution::cache::cache_unit::DefaultFileStatisticsCache">DefaultFileStatisticsCache</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.get" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.get" class="fn">get</a>(&self, k: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\>

Get `Statistics` for file location.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.get_with_extra" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.get_with_extra" class="fn">get_with_extra</a>( &self, k: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, e: &\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html" class="struct" title="struct datafusion::execution::cache::cache_unit::DefaultFileStatisticsCache">DefaultFileStatisticsCache</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html" class="trait" title="trait datafusion::execution::cache::CacheAccessor">CacheAccessor</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#associatedtype.Extra" class="associatedtype" title="type datafusion::execution::cache::CacheAccessor::Extra">Extra</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\>

Get `Statistics` for file location. Returns None if file has changed or not found.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.put" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.put" class="fn">put</a>(&self, \_key: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, \_value: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\>

Save collected file statistics

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#associatedtype.Extra" class="anchor">§</a>

#### type <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#associatedtype.Extra" class="associatedtype">Extra</a> = <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/struct.ObjectMeta.html" class="struct" title="struct object_store::ObjectMeta">ObjectMeta</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.put_with_extra" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.put_with_extra" class="fn">put_with_extra</a>( &self, key: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, value: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>, e: &\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html" class="struct" title="struct datafusion::execution::cache::cache_unit::DefaultFileStatisticsCache">DefaultFileStatisticsCache</a> as <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html" class="trait" title="trait datafusion::execution::cache::CacheAccessor">CacheAccessor</a>\<<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\>\>::<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#associatedtype.Extra" class="associatedtype" title="type datafusion::execution::cache::CacheAccessor::Extra">Extra</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\>

Put value into cache. Returns the old value associated with the key if there was one.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.remove" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.remove" class="fn">remove</a>(&mut self, k: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/struct.Statistics.html" class="struct" title="struct datafusion::common::Statistics">Statistics</a>\>\>

Remove an entry from the cache, returning value if they existed in the map.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.contains_key" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.contains_key" class="fn">contains_key</a>(&self, k: &<a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if the cache contains a specific key.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.len" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Fetch the total number of cache entries.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.clear" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.clear" class="fn">clear</a>(&self)

Remove all entries from the cache.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.name" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#tymethod.name" class="fn">name</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Return the cache name.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.is_empty" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/trait.CacheAccessor.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Check if the Cache collection is empty or not.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#impl-Default-for-DefaultFileStatisticsCache" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html" class="struct" title="struct datafusion::execution::cache::cache_unit::DefaultFileStatisticsCache">DefaultFileStatisticsCache</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html" class="struct" title="struct datafusion::execution::cache::cache_unit::DefaultFileStatisticsCache">DefaultFileStatisticsCache</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_unit/struct.DefaultFileStatisticsCache.html#blanket-implementations" class="anchor">§</a>
