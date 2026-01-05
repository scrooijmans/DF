# AsyncCatalogProviderList in datafusion::catalog - Rust

## Trait AsyncCatalogProviderList 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#336)

```
pub trait AsyncCatalogProviderList: Send + Sync {
    // Required method
    fn catalog<'life0, 'life1, 'async_trait>(
        &'life0 self,
        name: &'life1 str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<Arc<dyn AsyncCatalogProvider>>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             Self: 'async_trait;

    // Provided method
    fn resolve<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        references: &'life1 [TableReference],
        config: &'life2 SessionConfig,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn CatalogProviderList>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             Self: 'async_trait { ... }
}
```

Expand description

A trait for catalog provider lists that must resolve catalogs asynchronously

The [`CatalogProviderList::catalog`](about:blank/trait.CatalogProviderList.html#tymethod.catalog "method datafusion::catalog::CatalogProviderList::catalog") method is synchronous because asynchronous operations should not be used during planning. This trait makes it easy to lookup catalog references once and cache them for future planning use. See [`AsyncSchemaProvider`](trait.AsyncSchemaProvider.html "trait datafusion::catalog::AsyncSchemaProvider") for more details on motivation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#338)

Lookup a catalog in the provider

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#335)

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of catalog providers, schema providers, and table providers. This cache will be returned as a synchronous CatalogProvider that can be used to plan and execute a query containing the given references.

This cache is intended to be short-lived for the execution of a single query. There is no mechanism for refresh or eviction of stale entries.
