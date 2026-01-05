# Trait ObjectStoreRegistry Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/object_store.rs.html#149" class="src">Source</a>

``` rust
pub trait ObjectStoreRegistry:
    Send
    + Sync
    + Debug
    + 'static {
    // Required methods
    fn register_store(
        &self,
        url: &Url,
        store: Arc<dyn ObjectStore>,
    ) -> Option<Arc<dyn ObjectStore>>;
    fn get_store(
        &self,
        url: &Url,
    ) -> Result<Arc<dyn ObjectStore>, DataFusionError>;
}
```

Expand description

[`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry") maps a URL to an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instance, and allows DataFusion to read from different [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") instances. For example DataFusion might be configured so that

1.  `s3://my_bucket/lineitem/` mapped to the `/lineitem` path on an AWS S3 object store bound to `my_bucket`

2.  `s3://my_other_bucket/lineitem/` mapped to the (same) `/lineitem` path on a *different* AWS S3 object store bound to `my_other_bucket`

When given a [`ListingTableUrl`](https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTableUrl.html), DataFusion tries to find an appropriate [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore"). For example

``` sql
create external table unicorns stored as parquet location 's3://my_bucket/lineitem/';
```

In this particular case, the url `s3://my_bucket/lineitem/` will be provided to [`ObjectStoreRegistry::get_store`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.get_store "method datafusion::datasource::object_store::ObjectStoreRegistry::get_store") and one of three things will happen:

- If an [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") has been registered with [`ObjectStoreRegistry::register_store`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.register_store "method datafusion::datasource::object_store::ObjectStoreRegistry::register_store") with `s3://my_bucket`, that [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") will be returned

- If an AWS S3 object store can be ad-hoc discovered by the url `s3://my_bucket/lineitem/`, this object store will be registered with key `s3://my_bucket` and returned.

- Otherwise an error will be returned, indicating that no suitable [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") could be found

This allows for two different use-cases:

1.  Systems where object store buckets are explicitly created using DDL, can register these buckets using [`ObjectStoreRegistry::register_store`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.register_store "method datafusion::datasource::object_store::ObjectStoreRegistry::register_store")

2.  Systems relying on ad-hoc discovery, without corresponding DDL, can create [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") lazily by providing a custom implementation of [`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry")

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.register_store" class="fn">register_store</a>( &self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>\>

If a store with the same key existed before, it is replaced and returned

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.get_store" class="fn">get_store</a>(&self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get a suitable store for the provided URL. For example:

- URL with scheme `file:///` or no scheme will return the default LocalFS store
- URL with scheme `s3://bucket/` will return the S3 store
- URL with scheme `hdfs://hostname:port/` will return the hdfs store

If no [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") found for the `url`, ad-hoc discovery may be executed depending on the `url` and [`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry") implementation. An [`ObjectStore`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html "trait object_store::ObjectStore") may be lazily created and registered.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#impl-ObjectStoreRegistry-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html" class="trait" title="trait datafusion::datasource::object_store::ObjectStoreRegistry">ObjectStoreRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::datasource::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

Stores are registered based on the scheme, host and port of the provided URL with a [`LocalFileSystem::new`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/local/struct.LocalFileSystem.html#method.new "associated function object_store::local::LocalFileSystem::new") automatically registered for `file://` (if the target arch is not `wasm32`).

For example:

- `file:///my_path` will return the default LocalFS store
- `s3://bucket/path` will return a store registered with `s3://bucket` if any
- `hdfs://host:port/path` will return a store registered with `hdfs://host:port` if any
