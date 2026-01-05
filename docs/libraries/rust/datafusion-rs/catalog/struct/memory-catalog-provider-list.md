# MemoryCatalogProviderList in datafusion::catalog - Rust

## Struct MemoryCatalogProviderList 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#29)

```
pub struct MemoryCatalogProviderList {
    pub catalogs: DashMap<String, Arc<dyn CatalogProvider>>,
}
```

Expand description

Simple in-memory list of catalogs

Collection of catalogs containing schemas and ultimately TableProviders

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#34)
[§](#impl-MemoryCatalogProviderList)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#36)

Instantiates a new `MemoryCatalogProviderList` with an empty collection of catalogs

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#49)
[§](#impl-CatalogProviderList-for-MemoryCatalogProviderList)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#50)
[§](#method.as_any)

Returns the catalog list as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#54-58)
[§](#method.register_catalog)

Adds a new catalog to this catalog list If a catalog of the same name existed before, it is replaced in the list and returned.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#62)
[§](#method.catalog_names)

Retrieves the list of available catalog names

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#66)
[§](#method.catalog)

Retrieves a specific catalog by name, provided it exists.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#28)
[§](#impl-Debug-for-MemoryCatalogProviderList)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#43)
[§](#impl-Default-for-MemoryCatalogProviderList)

[§](#impl-Freeze-for-MemoryCatalogProviderList)

[§](#impl-RefUnwindSafe-for-MemoryCatalogProviderList)

[§](#impl-Send-for-MemoryCatalogProviderList)

[§](#impl-Sync-for-MemoryCatalogProviderList)

[§](#impl-Unpin-for-MemoryCatalogProviderList)

[§](#impl-UnwindSafe-for-MemoryCatalogProviderList)
