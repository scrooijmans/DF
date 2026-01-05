# Struct MemoryCatalogProvider Copy item path

<a href="https://docs.rs/datafusion-catalog/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_catalog/memory/catalog.rs.html#73" class="src">Source</a>

``` rust
pub struct MemoryCatalogProvider { /* private fields */ }
```

Expand description

Simple in-memory implementation of a catalog.

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#impl-MemoryCatalogProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>

Instantiates a new MemoryCatalogProvider with an empty collection of schemas.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#impl-CatalogProvider-for-MemoryCatalogProvider" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html" class="trait" title="trait datafusion::catalog::CatalogProvider">CatalogProvider</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.as_any" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#tymethod.as_any" class="fn">as_any</a>(&self) -\> &(dyn <a href="https://doc.rust-lang.org/nightly/core/any/trait.Any.html" class="trait" title="trait core::any::Any">Any</a> + 'static)

Returns the catalog provider as [`Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") so that it can be downcast to a specific implementation.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.schema_names" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#tymethod.schema_names" class="fn">schema_names</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Retrieves the list of available schema names in this catalog.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#tymethod.schema" class="fn">schema</a>(&self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>\>

Retrieves a specific schema from the catalog by name, provided it exists.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.register_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#method.register_schema" class="fn">register_schema</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, schema: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Adds a new schema to this catalog. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#method.register_schema)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.deregister_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#method.deregister_schema" class="fn">deregister_schema</a>( &self, name: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, cascade: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.SchemaProvider.html" class="trait" title="trait datafusion::catalog::SchemaProvider">SchemaProvider</a>\>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Removes a schema from this catalog. Implementations of this method should return errors if the schema exists but cannot be dropped. For example, in DataFusion’s default in-memory catalog, `MemoryCatalogProvider`, a non-empty schema will only be successfully dropped when `cascade` is true. This is equivalent to how DROP SCHEMA works in PostgreSQL. [Read more](https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.CatalogProvider.html#method.deregister_schema)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#impl-Debug-for-MemoryCatalogProvider" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#impl-Default-for-MemoryCatalogProvider" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html" class="struct" title="struct datafusion::catalog::MemoryCatalogProvider">MemoryCatalogProvider</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/struct.MemoryCatalogProvider.html#blanket-implementations" class="anchor">§</a>
