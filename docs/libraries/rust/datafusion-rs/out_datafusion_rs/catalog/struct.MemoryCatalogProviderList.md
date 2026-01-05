# Struct MemoryCatalogProviderList Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#29" class="src">Source</a>

``` rust
pub struct MemoryCatalogProviderList {
    pub catalogs: DashMap<String, Arc<dyn CatalogProvider>>,
}
```

Expand description

Simple in-memory list of catalogs

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#structfield.catalogs" class="anchor field">§</a>`catalogs: `<a href="https://docs.rs/dashmap/6.1.0/x86_64-unknown-linux-gnu/dashmap/struct.DashMap.html" class="struct" title="struct dashmap::DashMap"><code>DashMap</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc"><code>Arc</code></a>`<dyn `<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider"><code>CatalogProvider</code></a>`>>`

Collection of catalogs containing schemas and ultimately TableProviders

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#impl-MemoryCatalogProviderList" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>

Instantiates a new `MemoryCatalogProviderList` with an empty collection of catalogs

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#impl-CatalogProviderList-for-MemoryCatalogProviderList" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the catalog list as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#method.register_catalog" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.register_catalog" class="fn">register_catalog</a>( &self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, catalog: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Adds a new catalog to this catalog list If a catalog of the same name existed before, it is replaced in the list and returned.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#method.catalog_names" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.catalog_names" class="fn">catalog_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available catalog names

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#method.catalog" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.catalog" class="fn">catalog</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Retrieves a specific catalog by name, provided it exists.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#impl-Debug-for-MemoryCatalogProviderList" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#impl-Default-for-MemoryCatalogProviderList" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProviderList">MemoryCatalogProviderList</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProviderList.html#blanket-implementations" class="anchor">§</a>
