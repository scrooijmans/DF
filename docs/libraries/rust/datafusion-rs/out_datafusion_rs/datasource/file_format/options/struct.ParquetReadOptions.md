# Struct ParquetReadOptions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/file_format/options.rs.html#235-257" class="src">Source</a>

``` rust
pub struct ParquetReadOptions<'a> {
    pub file_extension: &'a str,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub parquet_pruning: Option<bool>,
    pub skip_metadata: Option<bool>,
    pub schema: Option<&'a Schema>,
    pub file_sort_order: Vec<Vec<SortExpr>>,
    pub file_decryption_properties: Option<ConfigFileDecryptionProperties>,
}
```

Expand description

Options that control the reading of Parquet files.

Note this structure is supplied when a datasource is created and can not not vary from statement to statement. For settings that can vary statement to statement see [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions").

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#structfield.file_extension" class="anchor field">§</a>`file_extension: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

File extension; only files with this extension are selected for data input. Defaults to “.parquet”.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#structfield.table_partition_cols" class="anchor field">§</a>`table_partition_cols: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>`)>`

Partition Columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#structfield.parquet_pruning" class="anchor field">§</a>`parquet_pruning: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Should the parquet reader use the predicate to prune row groups? If None, uses value in SessionConfig

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#structfield.skip_metadata" class="anchor field">§</a>`skip_metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>`>`

Should the parquet reader to skip any metadata that may be in the file Schema? This can help avoid schema conflicts due to metadata.

If None specified, uses value in SessionConfig

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<&'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema"><code>Schema</code></a>`>`

An optional schema representing the parquet files. If None, parquet reader will try to infer it based on data in file.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#structfield.file_sort_order" class="anchor field">§</a>`file_sort_order: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr"><code>SortExpr</code></a>`>>`

Indicates how the file is sorted

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#structfield.file_decryption_properties" class="anchor field">§</a>`file_decryption_properties: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties"><code>ConfigFileDecryptionProperties</code></a>`>`

Properties for decryption of Parquet files that use modular encryption

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#impl-ParquetReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions">ParquetReadOptions</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.new" class="fn">new</a>() -\> Self

Create a new ParquetReadOptions with default values

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.file_extension" class="fn">file_extension</a>(self, file_extension: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Specify file_extension

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.parquet_pruning" class="fn">parquet_pruning</a>(self, parquet_pruning: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Specify parquet_pruning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.skip_metadata" class="fn">skip_metadata</a>(self, skip_metadata: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Tell the parquet reader to skip any metadata that may be in the file Schema. This can help avoid schema conflicts due to metadata. Defaults to true.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.schema" class="fn">schema</a>(self, schema: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> Self

Specify schema to use for parquet read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.table_partition_cols" class="fn">table_partition_cols</a>( self, table_partition_cols: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>)\>, ) -\> Self

Specify table_partition_cols for partition pruning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.file_sort_order" class="fn">file_sort_order</a>(self, file_sort_order: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">SortExpr</a>\>\>) -\> Self

Configure if file has known sort order

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.file_decryption_properties" class="fn">file_decryption_properties</a>( self, file_decryption_properties: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigFileDecryptionProperties.html" class="struct" title="struct datafusion::config::ConfigFileDecryptionProperties">ConfigFileDecryptionProperties</a>, ) -\> Self

Configure file decryption properties for reading encrypted Parquet files

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#impl-Clone-for-ParquetReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions">ParquetReadOptions</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions">ParquetReadOptions</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#impl-Default-for-ParquetReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions">ParquetReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#impl-ReadOptions%3C&#39;_%3E-for-ParquetReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html" class="trait" title="trait datafusion::datasource::file_format::options::ReadOptions">ReadOptions</a>\<'\_\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ParquetReadOptions">ParquetReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.to_listing_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.to_listing_options" class="fn">to_listing_options</a>( &self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, table_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

Helper to convert these user facing options to `ListingTable` options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method.get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.get_resolved_schema" class="fn">get_resolved_schema</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, config: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Infer and resolve the schema from the files/sources provided.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#method._get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#method._get_resolved_schema" class="fn">_get_resolved_schema</a>\<'life0, 'async_trait\>( &'a self, config: &'life0 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'async_trait, 'a: 'async_trait, 'life0: 'async_trait,

helper function to reduce repetitive code. Infers the schema from sources if not provided. Infinite data sources not supported through this function.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ParquetReadOptions.html#blanket-implementations" class="anchor">§</a>
