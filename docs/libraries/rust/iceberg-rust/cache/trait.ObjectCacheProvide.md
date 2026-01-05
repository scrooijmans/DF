# Trait ObjectCacheProvide Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/cache.rs.html#47-52" class="src">Source</a>

``` rust
pub trait ObjectCacheProvide: Send + Sync {
    // Required methods
    fn manifest_cache(&self) -> &dyn ObjectCache<String, Arc<Manifest>>;
    fn manifest_list_cache(&self) -> &dyn ObjectCache<String, Arc<ManifestList>>;
}
```

Expand description

A trait for caching different in-memory objects used by iceberg.

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCacheProvide.html#notes" class="doc-anchor">§</a>Notes

ObjectCache will store deeply nested objects, such as `Manifest`, which contains `Schema`. Please ensure that the cache stores the object in memory as-is, without attempting to serialize it, as serialization could be extremely expensive.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCacheProvide.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCacheProvide.html#tymethod.manifest_cache" class="fn">manifest_cache</a>(&self) -\> &dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCache.html" class="trait" title="trait iceberg::cache::ObjectCache">ObjectCache</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Manifest.html" class="struct" title="struct iceberg::spec::Manifest">Manifest</a>\>\>

Gets a cache for manifests.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCacheProvide.html#tymethod.manifest_list_cache" class="fn">manifest_list_cache</a>(&self) -\> &dyn <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCache.html" class="trait" title="trait iceberg::cache::ObjectCache">ObjectCache</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.ManifestList.html" class="struct" title="struct iceberg::spec::ManifestList">ManifestList</a>\>\>

Gets a cache for manifest lists.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCacheProvide.html#implementors" class="anchor">§</a>
