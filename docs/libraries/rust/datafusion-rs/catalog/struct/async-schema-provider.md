# AsyncSchemaProvider in datafusion::catalog - Rust

## Trait AsyncSchemaProvider 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#198)

```
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

The [`SchemaProvider::table`](about:blank/trait.SchemaProvider.html#tymethod.table "method datafusion::catalog::SchemaProvider::table") method _is_ asynchronous. However, this is primarily for convenience and it is not a good idea for this method to be slow as this will cause poor planning performance.

It is a better idea to resolve the tables once and cache them in memory for the duration of planning. This trait helps implement that pattern.

After implementing this trait you can call the [`AsyncSchemaProvider::resolve`](about:blank/trait.AsyncSchemaProvider.html#method.resolve "method datafusion::catalog::AsyncSchemaProvider::resolve") method to get an `Arc<dyn SchemaProvider>` that contains a cached copy of the referenced tables. The `resolve` method can be slow and asynchronous as it is only called once, before planning.

See the [remote_catalog.rs](https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/remote_catalog.rs) for an end to end example

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#200)

Lookup a table in the schema provider

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/async.rs.html#197)

Creates a cached provider that can be used to execute a query containing given references

This method will walk through the references and look them up once, creating a cache of table providers. This cache will be returned as a synchronous TableProvider that can be used to plan and execute a query containing the given references.

This cache is intended to be short-lived for the execution of a single query. There is no mechanism for refresh or eviction of stale entries.

See the [`AsyncSchemaProvider`](trait.AsyncSchemaProvider.html "trait datafusion::catalog::AsyncSchemaProvider") documentation for additional details
