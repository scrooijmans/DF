# Trait ObjectStoreRegistry Copy item path

<a href="https://docs.rs/object_store/latest/src/object_store/registry.rs.html#28-85" class="src">Source</a>

``` rust
pub trait ObjectStoreRegistry:
    Send
    + Sync
    + Debug
    + 'static {
    // Required methods
    fn register(
        &self,
        url: Url,
        store: Arc<dyn ObjectStore>,
    ) -> Option<Arc<dyn ObjectStore>>;
    fn resolve(&self, url: &Url) -> Result<(Arc<dyn ObjectStore>, Path)>;
}
```

Expand description

[`ObjectStoreRegistry`](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html "trait object_store::registry::ObjectStoreRegistry") maps a URL to an [`ObjectStore`](https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance

## Required Methods<a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#tymethod.register" class="fn">register</a>( &self, url: <a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>\>

Register a new store for the provided store URL

If a store with the same URL existed before, it is replaced and returned

#### fn <a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#tymethod.resolve" class="fn">resolve</a>(&self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>) -\> <a href="https://docs.rs/object_store/latest/object_store/type.Result.html" class="type" title="type object_store::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/latest/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, <a href="https://docs.rs/object_store/latest/object_store/path/struct.Path.html" class="struct" title="struct object_store::path::Path">Path</a>)\>

Resolve an object URL

If [`ObjectStoreRegistry::register`](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#tymethod.register "method object_store::registry::ObjectStoreRegistry::register") has been called with a URL with the same scheme, and authority as the object URL, and a path that is a prefix of the object URL’s, it should be returned along with the trailing path. Paths should be matched on a path segment basis, and in the event of multiple possibilities the longest path match should be returned.

If a store hasn’t been registered, an [`ObjectStoreRegistry`](https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html "trait object_store::registry::ObjectStoreRegistry") may lazily create one if the URL is understood

For example

``` rust
let registry = DefaultObjectStoreRegistry::new();

let bucket1 = Arc::new(InMemory::new()) as Arc<dyn ObjectStore>;
let base = Url::parse("s3://bucket1/").unwrap();
registry.register(base, bucket1.clone());

let url = Url::parse("s3://bucket1/path/to/object").unwrap();
let (ret, path) = registry.resolve(&url).unwrap();
assert_eq!(path.as_ref(), "path/to/object");
assert!(Arc::ptr_eq(&ret, &bucket1));

let bucket2 = Arc::new(InMemory::new()) as Arc<dyn ObjectStore>;
let base = Url::parse("https://s3.region.amazonaws.com/bucket").unwrap();
registry.register(base, bucket2.clone());

let url = Url::parse("https://s3.region.amazonaws.com/bucket/path/to/object").unwrap();
let (ret, path) = registry.resolve(&url).unwrap();
assert_eq!(path.as_ref(), "path/to/object");
assert!(Arc::ptr_eq(&ret, &bucket2));

let bucket3 = Arc::new(PrefixStore::new(InMemory::new(), "path")) as Arc<dyn ObjectStore>;
let base = Url::parse("https://s3.region.amazonaws.com/bucket/path").unwrap();
registry.register(base, bucket3.clone());

let url = Url::parse("https://s3.region.amazonaws.com/bucket/path/to/object").unwrap();
let (ret, path) = registry.resolve(&url).unwrap();
assert_eq!(path.as_ref(), "to/object");
assert!(Arc::ptr_eq(&ret, &bucket3));
```

## Implementors<a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html#impl-ObjectStoreRegistry-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/object_store/latest/object_store/registry/trait.ObjectStoreRegistry.html" class="trait" title="trait object_store::registry::ObjectStoreRegistry">ObjectStoreRegistry</a> for <a href="https://docs.rs/object_store/latest/object_store/registry/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct object_store::registry::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>
