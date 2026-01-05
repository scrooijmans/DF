# Struct ListingTableConfig Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/listing/table.rs.html#87-105" class="src">Source</a>

``` rust
pub struct ListingTableConfig {
    pub table_paths: Vec<ListingTableUrl>,
    pub file_schema: Option<SchemaRef>,
    pub options: Option<ListingOptions>,
    /* private fields */
}
```

Expand description

Configuration for creating a [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#schema-evolution-support" class="doc-anchor">§</a>Schema Evolution Support

This configuration supports schema evolution through the optional [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory"). You might want to override the default factory when you need:

- **Type coercion requirements**: When you need custom logic for converting between different Arrow data types (e.g., Int32 ↔ Int64, Utf8 ↔ LargeUtf8)
- **Column mapping**: You need to map columns with a legacy name to a new name
- **Custom handling of missing columns**: By default they are filled in with nulls, but you may e.g. want to fill them in with `0` or `""`.

If not specified, a [`DefaultSchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/struct.DefaultSchemaAdapterFactory.html "struct datafusion::datasource::schema_adapter::DefaultSchemaAdapterFactory") will be used, which handles basic schema compatibility cases.

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#structfield.table_paths" class="anchor field">§</a>`table_paths: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl"><code>ListingTableUrl</code></a>`>`

Paths on the `ObjectStore` for creating `ListingTable`. They should share the same schema and object store.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#structfield.file_schema" class="anchor field">§</a>`file_schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef"><code>SchemaRef</code></a>`>`

Optional `SchemaRef` for the to be created `ListingTable`.

See details on [`ListingTableConfig::with_schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_schema "method datafusion::datasource::listing::ListingTableConfig::with_schema")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#structfield.options" class="anchor field">§</a>`options: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions"><code>ListingOptions</code></a>`>`

Optional [`ListingOptions`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html "struct datafusion::datasource::listing::ListingOptions") for the to be created [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable").

See details on [`ListingTableConfig::with_listing_options`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_listing_options "method datafusion::datasource::listing::ListingTableConfig::with_listing_options")

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#impl-ListingTableConfig" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.new" class="fn">new</a>(table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>) -\> Self

Creates new [`ListingTableConfig`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html "struct datafusion::datasource::listing::ListingTableConfig") for reading the specified URL

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.new_with_multi_paths" class="fn">new_with_multi_paths</a>(table_paths: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>\>) -\> Self

Creates new [`ListingTableConfig`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html "struct datafusion::datasource::listing::ListingTableConfig") with multiple table paths.

See [`Self::infer_options`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_options "method datafusion::datasource::listing::ListingTableConfig::infer_options") for details on what happens with multiple paths

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.schema_source" class="fn">schema_source</a>(&self) -\> SchemaSource

Returns the source of the schema for this configuration

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_schema" class="fn">with_schema</a>(self, schema: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>) -\> Self

Set the `schema` for the overall [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

[`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable") will automatically coerce, when possible, the schema for individual files to match this schema.

If a schema is not provided, it is inferred using [`Self::infer_schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_schema "method datafusion::datasource::listing::ListingTableConfig::infer_schema").

If the schema is provided, it must contain only the fields in the file without the table partitioning columns.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#example-specifying-table-schema" class="doc-anchor">§</a>Example: Specifying Table Schema

``` rust
let schema = Arc::new(Schema::new(vec![
    Field::new("id", DataType::Int64, false),
    Field::new("name", DataType::Utf8, true),
]));

let config = ListingTableConfig::new(table_paths)
    .with_listing_options(listing_options)  // Set options first
    .with_schema(schema);                    // Then set schema
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_listing_options" class="fn">with_listing_options</a>(self, listing_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>) -\> Self

Add `listing_options` to [`ListingTableConfig`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html "struct datafusion::datasource::listing::ListingTableConfig")

If not provided, format and other options are inferred via [`Self::infer_options`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_options "method datafusion::datasource::listing::ListingTableConfig::infer_options").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#example-configuring-parquet-files-with-custom-options" class="doc-anchor">§</a>Example: Configuring Parquet Files with Custom Options

``` rust
let options = ListingOptions::new(Arc::new(ParquetFormat::default()))
    .with_file_extension(".parquet")
    .with_collect_stat(true);

let config = ListingTableConfig::new(table_paths)
    .with_listing_options(options);  // Configure file format and options
```

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_options" class="fn">infer_options</a>(self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Infer `ListingOptions` based on `table_path` and file suffix.

The format is inferred based on the first `table_path`.

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_schema" class="fn">infer_schema</a>(self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Infer the [`SchemaRef`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html "type datafusion::common::arrow::datatypes::SchemaRef") based on `table_path`s.

This method infers the table schema using the first `table_path`. See [`ListingOptions::infer_schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html#method.infer_schema "method datafusion::datasource::listing::ListingOptions::infer_schema") for more details

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#errors" class="doc-anchor">§</a>Errors

- if `self.options` is not set. See [`Self::with_listing_options`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_listing_options "method datafusion::datasource::listing::ListingTableConfig::with_listing_options")

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer" class="fn">infer</a>(self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Convenience method to call both [`Self::infer_options`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_options "method datafusion::datasource::listing::ListingTableConfig::infer_options") and [`Self::infer_schema`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_schema "method datafusion::datasource::listing::ListingTableConfig::infer_schema")

#### pub async fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.infer_partitions_from_path" class="fn">infer_partitions_from_path</a>( self, state: &dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/catalog/trait.Session.html" class="trait" title="trait datafusion::catalog::Session">Session</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<Self\>

Infer the partition columns from `table_paths`.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#errors-1" class="doc-anchor">§</a>Errors

- if `self.options` is not set. See [`Self::with_listing_options`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_listing_options "method datafusion::datasource::listing::ListingTableConfig::with_listing_options")

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_schema_adapter_factory" class="fn">with_schema_adapter_factory</a>( self, schema_adapter_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>, ) -\> Self

Set the [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") for the [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

The schema adapter factory is used to create schema adapters that can handle schema evolution and type conversions when reading files with different schemas than the table schema.

If not provided, a default schema adapter factory will be used.

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#example-custom-schema-adapter-for-type-coercion" class="doc-anchor">§</a>Example: Custom Schema Adapter for Type Coercion

``` rust
let config = ListingTableConfig::new(table_paths)
    .with_listing_options(listing_options)
    .with_schema(table_schema)
    .with_schema_adapter_factory(Arc::new(MySchemaAdapterFactory));
```

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.schema_adapter_factory" class="fn">schema_adapter_factory</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html" class="trait" title="trait datafusion::datasource::schema_adapter::SchemaAdapterFactory">SchemaAdapterFactory</a>\>\>

Get the [`SchemaAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/schema_adapter/trait.SchemaAdapterFactory.html "trait datafusion::datasource::schema_adapter::SchemaAdapterFactory") for this configuration

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.with_expr_adapter_factory" class="fn">with_expr_adapter_factory</a>( self, expr_adapter_factory: <a href="https://doc.rust-lang.org/nightly/alloc/sync/struct.Arc.html" class="struct" title="struct alloc::sync::Arc">Arc</a>\<dyn <a href="https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html" class="trait" title="trait datafusion::physical_expr_adapter::PhysicalExprAdapterFactory">PhysicalExprAdapterFactory</a>\>, ) -\> Self

Set the [`PhysicalExprAdapterFactory`](https://docs.rs/datafusion/50.2.0/datafusion/physical_expr_adapter/trait.PhysicalExprAdapterFactory.html "trait datafusion::physical_expr_adapter::PhysicalExprAdapterFactory") for the [`ListingTable`](https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTable.html "struct datafusion::datasource::listing::ListingTable")

The expression adapter factory is used to create physical expression adapters that can handle schema evolution and type conversions when evaluating expressions with different schemas than the table schema.

If not provided, a default physical expression adapter factory will be used unless a custom `SchemaAdapterFactory` is set, in which case only the `SchemaAdapterFactory` will be used.

See <https://github.com/apache/datafusion/issues/16800> for details on this transition.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#impl-Clone-for-ListingTableConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#impl-Debug-for-ListingTableConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#impl-Default-for-ListingTableConfig" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html" class="struct" title="struct datafusion::datasource::listing::ListingTableConfig">ListingTableConfig</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableConfig.html#blanket-implementations" class="anchor">§</a>
