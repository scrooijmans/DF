# TableProviderFactory in datafusion::catalog - Rust

## Trait TableProviderFactory 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#307)

```
pub trait TableProviderFactory:
    Debug
    + Sync
    + Send {
    // Required method
    fn create<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        state: &'life1 dyn Session,
        cmd: &'life2 CreateExternalTable,
    ) -> Pin<Box<dyn Future<Output = Result<Arc<dyn TableProvider>, DataFusionError>> + Send + 'async_trait>>
       where 'life0: 'async_trait,
             'life1: 'async_trait,
             'life2: 'async_trait,
             Self: 'async_trait;
}
```

Expand description

A factory which creates [`TableProvider`](../datasource/trait.TableProvider.html "trait datafusion::datasource::TableProvider")s at runtime given a URL.

For example, this can be used to create a table “on the fly” from a directory of files only when that name is referenced.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/table.rs.html#309-313)

Create a TableProvider with the given url

[Source](about:blank/src/datafusion/datasource/listing_table_factory.rs.html#50-182)
[§](#impl-TableProviderFactory-for-ListingTableFactory)

[Source](about:blank/src/datafusion/datasource/provider.rs.html#50-68)
[§](#impl-TableProviderFactory-for-DefaultTableFactory)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/stream.rs.html#50)
[§](#impl-TableProviderFactory-for-StreamTableFactory)

[Source](about:blank/src/datafusion/test_util/mod.rs.html#179-190)
[§](#impl-TableProviderFactory-for-TestTableFactory)
