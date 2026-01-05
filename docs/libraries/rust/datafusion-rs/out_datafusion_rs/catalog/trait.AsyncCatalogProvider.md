# Trait AsyncCatalogProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#263" class="src">Source</a>

``` rust
pub trait AsyncCatalogProvider: Send + Sync {
    // Required method
    fn schema<'life0, 'life1, 'async_trait>(
        &'life0 self,
        name: &'life1 str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Arc<dyn AsyncSchemaProvider>>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;

    // Provided method
    fn resolve<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        references: &'life1 [TableReference],
        config: &'life2 SessionConfig,
        catalog_name: &'life3 str,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn CatalogProvider>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             'life3: 'async_trait,
             Self: 'async_trait { ... }
}
```

Expand description

A trait for catalog providers that must resolve schemas asynchronously

The [`CatalogProvider::schema`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#tymethod.schema "method datafusion::catalog::CatalogProvider::schema") method is synchronous because asynchronous operations should not be used during planning. This trait makes it easy to lookup schema references once and cache them for future planning use. See [`AsyncSchemaProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html "trait datafusion::catalog::AsyncSchemaProvider") for more details on motivation.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncCatalogProvider.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncCatalogProvider.html#tymethod.schema" class="fn">schema</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, name: &'life1 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncSchemaProvider.html" class="trait" title="trait datafusion::catalog::AsyncSchemaProvider">AsyncSchemaProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, Self: 'async_trait,

Lookup a schema in the provider

## Provided Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncCatalogProvider.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncCatalogProvider.html#method.resolve" class="fn">resolve</a>\<'life0, 'life1, 'life2, 'life3, 'async_trait\>( &'life0 self, references: &'life1 \[<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/enum.TableReference.html" class="enum" title="enum datafusion::common::TableReference">TableReference</a>\], config: &'life2 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, catalog_name: &'life3 <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where 'life0: 'async_trait, 'life1: 'async_trait, 'life2: 'async_trait, 'life3: 'async_trait, Self: 'async_trait,

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of schema providers (each with their own cache of table providers). This cache will be returned as a synchronous CatalogProvider that can be used to plan and execute a query containing the given references.

This cache is intended to be short-lived for the execution of a single query. There is no mechanism for refresh or eviction of stale entries.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.AsyncCatalogProvider.html#implementors" class="anchor">§</a>
