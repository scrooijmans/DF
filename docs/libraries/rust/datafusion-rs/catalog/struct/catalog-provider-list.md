# CatalogProviderList in datafusion::catalog - Rust

[datafusion](../index.html)::[catalog](index.html)

## Trait CatalogProviderList 

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/catalog.rs.html#159)

```
pub trait CatalogProviderList:
    Debug
    + Sync
    + Send {
    // Required methods
    fn as_any(&self) -> &(dyn Any + 'static);
    fn register_catalog(
        &self,
        name: String,
        catalog: Arc<dyn CatalogProvider>,
    ) -> Option<Arc<dyn CatalogProvider>>;
    fn catalog_names(&self) -> Vec<String>;
    fn catalog(&self, name: &str) -> Option<Arc<dyn CatalogProvider>>;
}
```

Expand description

Represent a list of named [`CatalogProvider`](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")s.

Please see the documentation on [`CatalogProvider`](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") for details of implementing a custom catalog.

## Required Methods[§](#required-methods)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/catalog.rs.html#162)

#### fn [as_any](#tymethod.as_any)(&self) -> &(dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + 'static)

Returns the catalog list as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/catalog.rs.html#166-170)

#### fn [register_catalog](#tymethod.register_catalog)( &self, name: [String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String"), catalog: [Arc](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")<dyn [CatalogProvider](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")\>, ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Arc](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")<dyn [CatalogProvider](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")\>>

Adds a new catalog to this catalog list If a catalog of the same name existed before, it is replaced in the list and returned.

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/catalog.rs.html#173)

#### fn [catalog_names](#tymethod.catalog_names)(&self) -> [Vec](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html "struct alloc::vec::Vec")<[String](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html "struct alloc::string::String")\>

Retrieves the list of available catalog names

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/catalog.rs.html#176)

#### fn [catalog](#tymethod.catalog)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Arc](https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html "struct alloc::sync::Arc")<dyn [CatalogProvider](trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")\>>

Retrieves a specific catalog by name, provided it exists.

## Implementors[§](#implementors)

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#44)
[§](#impl-CatalogProviderList-for-DynamicFileCatalog)

### impl [CatalogProviderList](trait.CatalogProviderList.html "trait datafusion::catalog::CatalogProviderList") for [DynamicFileCatalog](struct.DynamicFileCatalog.html "struct datafusion::catalog::DynamicFileCatalog")

[Source](https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#49)
[§](#impl-CatalogProviderList-for-MemoryCatalogProviderList)

### impl [CatalogProviderList](trait.CatalogProviderList.html "trait datafusion::catalog::CatalogProviderList") for [MemoryCatalogProviderList](struct.MemoryCatalogProviderList.html "struct datafusion::catalog::MemoryCatalogProviderList")
