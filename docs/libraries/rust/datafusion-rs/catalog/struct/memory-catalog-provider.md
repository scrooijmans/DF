# MemoryCatalogProvider in datafusion::catalog - Rust

## Struct MemoryCatalogProvider 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#73)

```
pub struct MemoryCatalogProvider { /* private fields */ }
```

Expand description

Simple in-memory implementation of a catalog.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#77)
[§](#impl-MemoryCatalogProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#79)

Instantiates a new MemoryCatalogProvider with an empty collection of schemas.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#92)
[§](#impl-CatalogProvider-for-MemoryCatalogProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#93)
[§](#method.as_any)

Returns the catalog provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#97)
[§](#method.schema_names)

Retrieves the list of available schema names in this catalog.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#101)
[§](#method.schema)

Retrieves a specific schema from the catalog by name, provided it exists.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#105-109)
[§](#method.register_schema)

Adds a new schema to this catalog. [Read more](about:blank/trait.CatalogProvider.html#method.register_schema)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#113-117)
[§](#method.deregister_schema)

Removes a schema from this catalog. Implementations of this method should return errors if the schema exists but cannot be dropped. For example, in DataFusion’s default in-memory catalog, `MemoryCatalogProvider`, a non-empty schema will only be successfully dropped when `cascade` is true. This is equivalent to how DROP SCHEMA works in PostgreSQL. [Read more](about:blank/trait.CatalogProvider.html#method.deregister_schema)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#72)
[§](#impl-Debug-for-MemoryCatalogProvider)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#86)
[§](#impl-Default-for-MemoryCatalogProvider)

[§](#impl-Freeze-for-MemoryCatalogProvider)

[§](#impl-RefUnwindSafe-for-MemoryCatalogProvider)

[§](#impl-Send-for-MemoryCatalogProvider)

[§](#impl-Sync-for-MemoryCatalogProvider)

[§](#impl-Unpin-for-MemoryCatalogProvider)

[§](#impl-UnwindSafe-for-MemoryCatalogProvider)
