# DynamicFileCatalog in datafusion::catalog - Rust

## Struct DynamicFileCatalog 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#28)

```
pub struct DynamicFileCatalog { /* private fields */ }
```

Expand description

Wrap another catalog provider list

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#44)
[§](#impl-CatalogProviderList-for-DynamicFileCatalog)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#45)
[§](#method.as_any)

Returns the catalog list as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#49-53)
[§](#method.register_catalog)

Adds a new catalog to this catalog list If a catalog of the same name existed before, it is replaced in the list and returned.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#57)
[§](#method.catalog_names)

Retrieves the list of available catalog names

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#61)
[§](#method.catalog)

Retrieves a specific catalog by name, provided it exists.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#27)
[§](#impl-Debug-for-DynamicFileCatalog)

[§](#impl-Freeze-for-DynamicFileCatalog)

[§](#impl-RefUnwindSafe-for-DynamicFileCatalog)

[§](#impl-Send-for-DynamicFileCatalog)

[§](#impl-Sync-for-DynamicFileCatalog)

[§](#impl-Unpin-for-DynamicFileCatalog)

[§](#impl-UnwindSafe-for-DynamicFileCatalog)
