# Trait CatalogProviderList Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/catalog.rs.html#159" class="src">Source</a>

``` rust
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

Represent a list of named [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider")s.

Please see the documentation on [`CatalogProvider`](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html "trait datafusion::catalog::CatalogProvider") for details of implementing a custom catalog.

## Required Methods<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the catalog list as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.register_catalog" class="fn">register_catalog</a>( &self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, catalog: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Adds a new catalog to this catalog list If a catalog of the same name existed before, it is replaced in the list and returned.

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.catalog_names" class="fn">catalog_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available catalog names

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.catalog" class="fn">catalog</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Retrieves a specific catalog by name, provided it exists.

## Implementors<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#impl-CatalogProviderList-for-DynamicFileCatalog" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html" class="struct" title="struct datafusion::catalog::DynamicFileCatalog">DynamicFileCatalog</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#impl-CatalogProviderList-for-MemoryCatalogProviderList" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>
