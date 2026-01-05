# Trait AsyncSchemaProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#198" class="src">Source</a>

``` rust
pub trait AsyncSchemaProvider: Send + Sync {
    // Required method
    fn table<'life0, 'life1, 'async_trait>(
        &'life0 self,
        name: &'life1 str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Arc<dyn TableProvider>>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;

    // Provided method
    fn resolve<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait>(
        &'life0 self,
        references: &'life1 [TableReference],
        config: &'life2 SessionConfig,
        catalog_name: &'life3 str,
        schema_name: &'life4 str,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn SchemaProvider>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             'life3: 'async_trait,
             'life4: 'async_trait,
             Self: 'async_trait { ... }
}
```

Expand description

A trait for schema providers that must resolve tables asynchronously

The [`SchemaProvider::table`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html#tymethod.table "method datafusion::catalog::SchemaProvider::table") method *is* asynchronous. However, this is primarily for convenience and it is not a good idea for this method to be slow as this will cause poor planning performance.

It is a better idea to resolve the tables once and cache them in memory for the duration of planning. This trait helps implement that pattern.

After implementing this trait you can call the [`AsyncSchemaProvider::resolve`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html#method.resolve "method datafusion::catalog::AsyncSchemaProvider::resolve") method to get an `Arc<dyn SchemaProvider>` that contains a cached copy of the referenced tables. The `resolve` method can be slow and asynchronous as it is only called once, before planning.

See the [remote_catalog.rs](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/remote_catalog.rs) for an end to end example

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html#tymethod.table" class="fn">table</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/trait.TableProvider.html" class="trait" title="trait datafusion::datasource::TableProvider">TableProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Lookup a table in the schema provider

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html#method.resolve" class="fn">resolve</a>\<'life0, 'life1, 'life2, 'life3, 'life4, 'async_trait\>( &'life0 self, references: &'life1 \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\], config: &'life2 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, catalog_name: &'life3 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, schema_name: &'life4 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, 'life3: 'async_trait, 'life4: 'async_trait, Self: 'async_trait,

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of table providers. This cache will be returned as a synchronous TableProvider that can be used to plan and execute a query containing the given references.

This cache is intended to be short-lived for the execution of a single query. There is no mechanism for refresh or eviction of stale entries.

See the [`AsyncSchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html "trait datafusion::catalog::AsyncSchemaProvider") documentation for additional details

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html#implementors" class="anchor">§</a>
