# Struct NdJsonReadOptions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/file_format/options.rs.html#428-444" class="src">Source</a>

``` rust
pub struct NdJsonReadOptions<'a> {
    pub schema: Option<&'a Schema>,
    pub schema_infer_max_records: usize,
    pub file_extension: &'a str,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub file_compression_type: FileCompressionType,
    pub infinite: bool,
    pub file_sort_order: Vec<Vec<SortExpr>>,
}
```

Expand description

Options that control the reading of Line-delimited JSON files (NDJson)

Note this structure is supplied when a datasource is created and can not not vary from statement to statement. For settings that can vary statement to statement see [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions").

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<&'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema"><code>Schema</code></a>`>`

The data source schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#structfield.schema_infer_max_records" class="anchor field">§</a>`schema_infer_max_records: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Max number of rows to read from JSON files for schema inference if needed. Defaults to `DEFAULT_SCHEMA_INFER_MAX_RECORD`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#structfield.file_extension" class="anchor field">§</a>`file_extension: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

File extension; only files with this extension are selected for data input. Defaults to `FileType::JSON.get_ext().as_str()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#structfield.table_partition_cols" class="anchor field">§</a>`table_partition_cols: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>`)>`

Partition Columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#structfield.file_compression_type" class="anchor field">§</a>`file_compression_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/file_compression_type/struct.FileCompressionType.html" class="struct" title="struct datafusion::datasource::file_format::file_compression_type::FileCompressionType"><code>FileCompressionType</code></a>

File compression type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#structfield.infinite" class="anchor field">§</a>`infinite: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Flag indicating whether this file may be unbounded (as in a FIFO file).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#structfield.file_sort_order" class="anchor field">§</a>`file_sort_order: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr"><code>SortExpr</code></a>`>>`

Indicates how the file is sorted

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#impl-NdJsonReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions">NdJsonReadOptions</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.table_partition_cols" class="fn">table_partition_cols</a>( self, table_partition_cols: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>)\>, ) -\> Self

Specify table_partition_cols for partition pruning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.file_extension" class="fn">file_extension</a>(self, file_extension: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Specify file_extension

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.mark_infinite" class="fn">mark_infinite</a>(self, infinite: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Configure mark_infinite setting

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.file_compression_type" class="fn">file_compression_type</a>( self, file_compression_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/file_compression_type/struct.FileCompressionType.html" class="struct" title="struct datafusion::datasource::file_format::file_compression_type::FileCompressionType">FileCompressionType</a>, ) -\> Self

Specify file_compression_type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.schema" class="fn">schema</a>(self, schema: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> Self

Specify schema to use for NdJson read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.file_sort_order" class="fn">file_sort_order</a>(self, file_sort_order: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">SortExpr</a>\>\>) -\> Self

Configure if file has known sort order

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#impl-Clone-for-NdJsonReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions">NdJsonReadOptions</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions">NdJsonReadOptions</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#impl-Default-for-NdJsonReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions">NdJsonReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#impl-ReadOptions%3C&#39;_%3E-for-NdJsonReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html" class="trait" title="trait datafusion::datasource::file_format::options::ReadOptions">ReadOptions</a>\<'\_\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::NdJsonReadOptions">NdJsonReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.to_listing_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.to_listing_options" class="fn">to_listing_options</a>( &self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, table_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

Helper to convert these user facing options to `ListingTable` options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method.get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.get_resolved_schema" class="fn">get_resolved_schema</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, config: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Infer and resolve the schema from the files/sources provided.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#method._get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#method._get_resolved_schema" class="fn">_get_resolved_schema</a>\<'life0, 'async_trait\>( &'a self, config: &'life0 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'async_trait, 'a: 'async_trait, 'life0: 'async_trait,

helper function to reduce repetitive code. Infers the schema from sources if not provided. Infinite data sources not supported through this function.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.NdJsonReadOptions.html#blanket-implementations" class="anchor">§</a>
