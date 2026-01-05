# Struct DynamicFileCatalog Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/dynamic_file/catalog.rs.html#28" class="src">Source</a>

``` rust
pub struct DynamicFileCatalog { /* private fields */ }
```

Expand description

Wrap another catalog provider list

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#impl-DynamicFileCatalog" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html" class="struct" title="struct datafusion::catalog::DynamicFileCatalog">DynamicFileCatalog</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#method.new" class="fn">new</a>( inner: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a>\>, factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.UrlTableFactory.html" class="trait" title="trait datafusion::catalog::UrlTableFactory">UrlTableFactory</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html" class="struct" title="struct datafusion::catalog::DynamicFileCatalog">DynamicFileCatalog</a>

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#impl-CatalogProviderList-for-DynamicFileCatalog" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html" class="trait" title="trait datafusion::catalog::CatalogProviderList">CatalogProviderList</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html" class="struct" title="struct datafusion::catalog::DynamicFileCatalog">DynamicFileCatalog</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the catalog list as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#method.register_catalog" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.register_catalog" class="fn">register_catalog</a>( &self, name: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, catalog: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Adds a new catalog to this catalog list If a catalog of the same name existed before, it is replaced in the list and returned.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#method.catalog_names" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.catalog_names" class="fn">catalog_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available catalog names

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#method.catalog" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProviderList.html#tymethod.catalog" class="fn">catalog</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a>\>\>

Retrieves a specific catalog by name, provided it exists.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#impl-Debug-for-DynamicFileCatalog" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html" class="struct" title="struct datafusion::catalog::DynamicFileCatalog">DynamicFileCatalog</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.DynamicFileCatalog.html#blanket-implementations" class="anchor">§</a>
