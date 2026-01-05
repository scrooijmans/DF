# Trait ObjectCache Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/cache.rs.html#32-37" class="src">Source</a>

``` rust
pub trait ObjectCache<K, V>: Send + Sync {
    // Required methods
    fn get(&self, key: &K) -> Option<V>;
    fn set(&self, key: K, value: V);
}
```

Expand description

A trait for caching in-memory objects of given type.

## <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCache.html#notes" class="doc-anchor">§</a>Notes

ObjectCache will store deeply nested objects, such as `Manifest`, which contains `Schema`. Please ensure that the cache stores the object in memory as-is, without attempting to serialize it, as serialization could be extremely expensive.

## Required Methods<a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCache.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCache.html#tymethod.get" class="fn">get</a>(&self, key: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;K</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<V\>

Gets an object from the cache by its key.

#### fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCache.html#tymethod.set" class="fn">set</a>(&self, key: K, value: V)

Sets an object in the cache with the given key and value.

## Implementors<a href="https://docs.rs/iceberg/0.7.0/iceberg/cache/trait.ObjectCache.html#implementors" class="anchor">§</a>
