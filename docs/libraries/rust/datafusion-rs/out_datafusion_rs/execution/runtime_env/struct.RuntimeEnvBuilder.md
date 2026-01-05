# Struct RuntimeEnvBuilder Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/runtime_env.rs.html#184" class="src">Source</a>

``` rust
pub struct RuntimeEnvBuilder {
    pub disk_manager: DiskManagerConfig,
    pub disk_manager_builder: Option<DiskManagerBuilder>,
    pub memory_pool: Option<Arc<dyn MemoryPool>>,
    pub cache_manager: CacheManagerConfig,
    pub object_store_registry: Arc<dyn ObjectStoreRegistry>,
    pub parquet_encryption_factory_registry: Arc<EncryptionFactoryRegistry>,
}
```

Expand description

Execution runtime configuration builder.

See example on [`RuntimeEnv`](https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html "struct datafusion::execution::runtime_env::RuntimeEnv")

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#structfield.disk_manager" class="anchor field">§</a>`disk_manager: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig"><code>DiskManagerConfig</code></a>

DiskManager to manage temporary disk file usage

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#structfield.disk_manager_builder" class="anchor field">§</a>`disk_manager_builder: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder"><code>DiskManagerBuilder</code></a>`>`

DiskManager builder to manager temporary disk file usage

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#structfield.memory_pool" class="anchor field">§</a>`memory_pool: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool"><code>MemoryPool</code></a>`>>`

[`MemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html "trait datafusion::execution::memory_pool::MemoryPool") from which to allocate memory

Defaults to using an [`UnboundedMemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.UnboundedMemoryPool.html "struct datafusion::execution::memory_pool::UnboundedMemoryPool") if `None`

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#structfield.cache_manager" class="anchor field">§</a>`cache_manager: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManagerConfig.html" class="struct" title="struct datafusion::execution::cache::cache_manager::CacheManagerConfig"><code>CacheManagerConfig</code></a>

CacheManager to manage cache data

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#structfield.object_store_registry" class="anchor field">§</a>`object_store_registry: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html" class="trait" title="trait datafusion::datasource::object_store::ObjectStoreRegistry"><code>ObjectStoreRegistry</code></a>`>`

ObjectStoreRegistry to get object store based on url

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#structfield.parquet_encryption_factory_registry" class="anchor field">§</a>`parquet_encryption_factory_registry: `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/parquet_encryption/struct.EncryptionFactoryRegistry.html" class="struct" title="struct datafusion::execution::parquet_encryption::EncryptionFactoryRegistry"><code>EncryptionFactoryRegistry</code></a>`>`

Parquet encryption factory registry

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#impl-RuntimeEnvBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

New with default values

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_disk_manager" class="fn">with_disk_manager</a>( self, disk_manager: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/enum.DiskManagerConfig.html" class="enum" title="enum datafusion::execution::disk_manager::DiskManagerConfig">DiskManagerConfig</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

👎Deprecated since 48.0.0: Use with_disk_manager_builder instead

Customize disk manager

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_disk_manager_builder" class="fn">with_disk_manager_builder</a>( self, disk_manager: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/disk_manager/struct.DiskManagerBuilder.html" class="struct" title="struct datafusion::execution::disk_manager::DiskManagerBuilder">DiskManagerBuilder</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Customize the disk manager builder

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_memory_pool" class="fn">with_memory_pool</a>( self, memory_pool: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/trait.MemoryPool.html" class="trait" title="trait datafusion::execution::memory_pool::MemoryPool">MemoryPool</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Customize memory policy

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_cache_manager" class="fn">with_cache_manager</a>( self, cache_manager: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/cache/cache_manager/struct.CacheManagerConfig.html" class="struct" title="struct datafusion::execution::cache::cache_manager::CacheManagerConfig">CacheManagerConfig</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Customize cache policy

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_object_store_registry" class="fn">with_object_store_registry</a>( self, object_store_registry: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html" class="trait" title="trait datafusion::datasource::object_store::ObjectStoreRegistry">ObjectStoreRegistry</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Customize object store registry

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_memory_limit" class="fn">with_memory_limit</a>( self, max_memory: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, memory_fraction: <a href="https://doc.rust-lang.org/nightly/std/primitive.f64.html" class="primitive">f64</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Specify the total memory to use while running the DataFusion plan to `max_memory * memory_fraction` in bytes.

This defaults to using [`GreedyMemoryPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.GreedyMemoryPool.html "struct datafusion::execution::memory_pool::GreedyMemoryPool") wrapped in the [`TrackConsumersPool`](https://docs.rs/datafusion/50.2.0/datafusion/execution/memory_pool/struct.TrackConsumersPool.html "struct datafusion::execution::memory_pool::TrackConsumersPool") with a maximum of 5 consumers.

Note DataFusion does not yet respect this limit in all cases.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_temp_file_path" class="fn">with_temp_file_path</a>(self, path: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Use the specified path to create any needed temporary files

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_max_temp_directory_size" class="fn">with_max_temp_directory_size</a>(self, size: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Specify a limit on the size of the temporary file directory in bytes

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.with_metadata_cache_limit" class="fn">with_metadata_cache_limit</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Specify the limit of the file-embedded metadata cache, in bytes.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.build" class="fn">build</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Build a RuntimeEnv

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.build_arc" class="fn">build_arc</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Convenience method to create a new `Arc<RuntimeEnv>`

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.from_runtime_env" class="fn">from_runtime_env</a>(runtime_env: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnv.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnv">RuntimeEnv</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Create a new RuntimeEnvBuilder from an existing RuntimeEnv

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.entries" class="fn">entries</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigEntry.html" class="struct" title="struct datafusion::config::ConfigEntry">ConfigEntry</a>\>

Returns a list of all available runtime configurations with their current values and descriptions

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.generate_config_markdown" class="fn">generate_config_markdown</a>() -\> <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>

Generate documentation that can be included in the user guide

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#impl-Clone-for-RuntimeEnvBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#impl-Default-for-RuntimeEnvBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html" class="struct" title="struct datafusion::execution::runtime_env::RuntimeEnvBuilder">RuntimeEnvBuilder</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/runtime_env/struct.RuntimeEnvBuilder.html#blanket-implementations" class="anchor">§</a>
