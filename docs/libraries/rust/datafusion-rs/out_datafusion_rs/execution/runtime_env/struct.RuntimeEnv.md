# Struct RuntimeEnv Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/runtime_env.rs.html#74" class="src">Source</a>

``` rust
pub struct RuntimeEnv {
    pub memory_pool: Arc<dyn MemoryPool>,
    pub disk_manager: Arc<DiskManager>,
    pub cache_manager: Arc<CacheManager>,
    pub object_store_registry: Arc<dyn ObjectStoreRegistry>,
    pub parquet_encryption_factory_registry: Arc<EncryptionFactoryRegistry>,
}
```

Expand description

Execution runtime environment that manages system resources such as memory, disk, cache and storage.

A [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv") can be created using [`RuntimeEnvBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html "struct datafusion::execution::runtime_env::RuntimeEnvBuilder") and has the following resource management functionality:

- [`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool"): Manage memory
- [`DiskManager`](https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html "struct datafusion::execution::DiskManager"): Manage temporary files on local disk
- [`CacheManager`](https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html "struct datafusion::execution::cache::cache_manager::CacheManager"): Manage temporary cache data during the session lifetime
- [`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry"): Manage mapping URLs to object store instances

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#example-create-default-runtimeenv" class="doc-anchor">§</a>Example: Create default `RuntimeEnv`

``` rust
let runtime_env = RuntimeEnv::default();
```

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#example-create-a-runtimeenv-from-runtimeenvbuilder-with-a-new-memory-pool" class="doc-anchor">§</a>Example: Create a `RuntimeEnv` from [`RuntimeEnvBuilder`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html "struct datafusion::execution::runtime_env::RuntimeEnvBuilder") with a new memory pool

``` rust
// restrict to using at most 100MB of memory
let pool_size = 100 * 1024 * 1024;
let runtime_env = RuntimeEnvBuilder::new()
  .with_memory_pool(Arc::new(GreedyMemoryPool::new(pool_size)))
  .build()
  .unwrap();
```

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#structfield.memory_pool" class="anchor field">§</a>`memory_pool: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool"><code>MemoryPool</code></a>`>`

Runtime memory management

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#structfield.disk_manager" class="anchor field">§</a>`disk_manager: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/struct.DiskManager.html" class="struct" title="struct datafusion::execution::DiskManager"><code>DiskManager</code></a>`>`

Manage temporary files during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#structfield.cache_manager" class="anchor field">§</a>`cache_manager: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManager.html" class="struct" title="struct datafusion::execution::cache::cache_manager::CacheManager"><code>CacheManager</code></a>`>`

Manage temporary cache during query execution

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#structfield.object_store_registry" class="anchor field">§</a>`object_store_registry: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html" class="trait" title="trait datafusion::datasource::object_store::ObjectStoreRegistry"><code>ObjectStoreRegistry</code></a>`>`

Object Store Registry

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#structfield.parquet_encryption_factory_registry" class="anchor field">§</a>`parquet_encryption_factory_registry: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry"><code>EncryptionFactoryRegistry</code></a>`>`

Parquet encryption factory registry

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#impl-RuntimeEnv" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.register_object_store" class="fn">register_object_store</a>( &self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, object_store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>\>

Registers a custom `ObjectStore` to be used with a specific url. This allows DataFusion to create external tables from urls that do not have built in support such as `hdfs://namenode:port/...`.

Returns the [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") previously registered for this scheme, if any.

See [`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry") for more details

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#example-register-local-file-system-object-store" class="doc-anchor">§</a>Example: Register local file system object store

``` rust
let url = Url::try_from("file://").unwrap();
let object_store = object_store::local::LocalFileSystem::new();
// register the object store with the runtime environment
runtime_env.register_object_store(&url, Arc::new(object_store));
```

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#example-register-remote-url-object-store-like-github" class="doc-anchor">§</a>Example: Register remote URL object store like [Github](https://github.com)

``` rust
// create a new object store via object_store::http::HttpBuilder;
let base_url = Url::parse("https://github.com").unwrap();
// (note this example can't depend on the http feature)
// let http_store = HttpBuilder::new()
//    .with_url(base_url.clone())
//    .build()
//    .unwrap();
// register the object store with the runtime environment
runtime_env.register_object_store(&base_url, Arc::new(http_store));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.object_store" class="fn">object_store</a>( &self, url: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html" class="trait" title="trait core::convert::AsRef">AsRef</a>\<<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Retrieves a `ObjectStore` instance for a url by consulting the registry. See [`ObjectStoreRegistry::get_store`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.get_store "method datafusion::datasource::object_store::ObjectStoreRegistry::get_store") for more details.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.register_parquet_encryption_factory" class="fn">register_parquet_encryption_factory</a>( &self, id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, encryption_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>\>\>

Register an [`EncryptionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html "trait datafusion::execution::parquet_encryption::EncryptionFactory") with an associated identifier that can be later used to configure encryption when reading or writing Parquet. If an encryption factory with the same identifier was already registered, it is replaced and returned.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.parquet_encryption_factory" class="fn">parquet_encryption_factory</a>( &self, id: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html" class="trait" title="trait datafusion::execution::parquet_encryption::EncryptionFactory">EncryptionFactory</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Retrieve an [`EncryptionFactory`](https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/trait.EncryptionFactory.html "trait datafusion::execution::parquet_encryption::EncryptionFactory") by its identifier

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#impl-Clone-for-RuntimeEnv" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#impl-Debug-for-RuntimeEnv" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#impl-Default-for-RuntimeEnv" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html#blanket-implementations" class="anchor">§</a>
