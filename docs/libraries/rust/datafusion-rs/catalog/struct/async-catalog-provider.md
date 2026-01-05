# AsyncCatalogProvider in datafusion::catalog - Rust

## Trait AsyncCatalogProvider 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#263)

```
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

The [`CatalogProvider::schema`](about:blank/trait.CatalogProvider.html#tymethod.schema "method datafusion::catalog::CatalogProvider::schema") method is synchronous because asynchronous operations should not be used during planning. This trait makes it easy to lookup schema references once and cache them for future planning use. See [`AsyncSchemaProvider`](trait.AsyncSchemaProvider.html "trait datafusion::catalog::AsyncSchemaProvider") for more details on motivation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#265)

Lookup a schema in the provider

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#262)

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of schema providers (each with their own cache of table providers). This cache will be returned as a synchronous CatalogProvider that can be used to plan and execute a query containing the given references.

This cache is intended to be short-lived for the execution of a single query. There is no mechanism for refresh or eviction of stale entries.
