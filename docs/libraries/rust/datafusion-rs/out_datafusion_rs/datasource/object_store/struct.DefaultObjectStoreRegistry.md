# Struct DefaultObjectStoreRegistry Copy item path

<a href="https://docs.rs/datafusion-execution/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_execution/object_store.rs.html#170" class="src">Source</a>

``` rust
pub struct DefaultObjectStoreRegistry { /* private fields */ }
```

Expand description

The default [`ObjectStoreRegistry`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html "trait datafusion::datasource::object_store::ObjectStoreRegistry")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#impl-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::datasource::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::datasource::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

This will register [`LocalFileSystem`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/local/struct.LocalFileSystem.html "struct object_store::local::LocalFileSystem") to handle `file://` paths

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#impl-Debug-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::datasource::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#impl-Default-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::datasource::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::datasource::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#impl-ObjectStoreRegistry-for-DefaultObjectStoreRegistry" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html" class="trait" title="trait datafusion::datasource::object_store::ObjectStoreRegistry">ObjectStoreRegistry</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html" class="struct" title="struct datafusion::datasource::object_store::DefaultObjectStoreRegistry">DefaultObjectStoreRegistry</a>

Stores are registered based on the scheme, host and port of the provided URL with a [`LocalFileSystem::new`](https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/local/struct.LocalFileSystem.html#method.new "associated function object_store::local::LocalFileSystem::new") automatically registered for `file://` (if the target arch is not `wasm32`).

For example:

- `file:///my_path` will return the default LocalFS store
- `s3://bucket/path` will return a store registered with `s3://bucket` if any
- `hdfs://host:port/path` will return a store registered with `hdfs://host:port` if any

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#method.register_store" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.register_store" class="fn">register_store</a>( &self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>, store: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>\>

If a store with the same key existed before, it is replaced and returned

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#method.get_store" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.get_store" class="fn">get_store</a>(&self, url: &<a href="https://docs.rs/url/2.5.7/x86_64-unknown-linux-gnu/url/struct.Url.html" class="struct" title="struct url::Url">Url</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/object_store/0.12.3/x86_64-unknown-linux-gnu/object_store/trait.ObjectStore.html" class="trait" title="trait object_store::ObjectStore">ObjectStore</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Get a suitable store for the provided URL. For example: [Read more](https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/trait.ObjectStoreRegistry.html#tymethod.get_store)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/object_store/struct.DefaultObjectStoreRegistry.html#blanket-implementations" class="anchor">§</a>
