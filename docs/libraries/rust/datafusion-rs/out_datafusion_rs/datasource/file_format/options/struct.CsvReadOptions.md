# Struct CsvReadOptions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/file_format/options.rs.html#54-94" class="src">Source</a>

``` rust
pub struct CsvReadOptions<'a> {Show 14 fields
    pub has_header: bool,
    pub delimiter: u8,
    pub quote: u8,
    pub terminator: Option<u8>,
    pub escape: Option<u8>,
    pub comment: Option<u8>,
    pub newlines_in_values: bool,
    pub schema: Option<&'a Schema>,
    pub schema_infer_max_records: usize,
    pub file_extension: &'a str,
    pub table_partition_cols: Vec<(String, DataType)>,
    pub file_compression_type: FileCompressionType,
    pub file_sort_order: Vec<Vec<SortExpr>>,
    pub null_regex: Option<String>,
}
```

Expand description

Options that control the reading of CSV files.

Note this structure is supplied when a datasource is created and can not not vary from statement to statement. For settings that can vary statement to statement see [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions").

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.has_header" class="anchor field">§</a>`has_header: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Does the CSV file have a header?

If schema inference is run on a file with no headers, default column names are created.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.delimiter" class="anchor field">§</a>`delimiter: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

An optional column delimiter. Defaults to `b','`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.quote" class="anchor field">§</a>`quote: `<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>

An optional quote character. Defaults to `b'"'`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.terminator" class="anchor field">§</a>`terminator: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`

An optional terminator character. Defaults to None (CRLF).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.escape" class="anchor field">§</a>`escape: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`

An optional escape character. Defaults to None.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.comment" class="anchor field">§</a>`comment: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive"><code>u8</code></a>`>`

If enabled, lines beginning with this byte are ignored.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.newlines_in_values" class="anchor field">§</a>`newlines_in_values: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a>

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as parallel file scanning. Setting this to `true` ensures that newlines in values are parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<&'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema"><code>Schema</code></a>`>`

An optional schema representing the CSV files. If None, CSV reader will try to infer it based on data in file.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.schema_infer_max_records" class="anchor field">§</a>`schema_infer_max_records: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Max number of rows to read from CSV files for schema inference if needed. Defaults to `DEFAULT_SCHEMA_INFER_MAX_RECORD`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.file_extension" class="anchor field">§</a>`file_extension: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

File extension; only files with this extension are selected for data input. Defaults to `FileType::CSV.get_ext().as_str()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.table_partition_cols" class="anchor field">§</a>`table_partition_cols: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>`)>`

Partition Columns

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.file_compression_type" class="anchor field">§</a>`file_compression_type: `<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/file_compression_type/struct.FileCompressionType.html" class="struct" title="struct datafusion::datasource::file_format::file_compression_type::FileCompressionType"><code>FileCompressionType</code></a>

File compression type

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.file_sort_order" class="anchor field">§</a>`file_sort_order: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr"><code>SortExpr</code></a>`>>`

Indicates how the file is sorted

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#structfield.null_regex" class="anchor field">§</a>`null_regex: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

Optional regex to match null values

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#impl-CsvReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions">CsvReadOptions</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.new" class="fn">new</a>() -\> Self

Create a CSV read option with default presets

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.has_header" class="fn">has_header</a>(self, has_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Configure has_header setting

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.comment" class="fn">comment</a>(self, comment: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify comment char to use for CSV read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.delimiter" class="fn">delimiter</a>(self, delimiter: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify delimiter to use for CSV read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.quote" class="fn">quote</a>(self, quote: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify quote to use for CSV read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.terminator" class="fn">terminator</a>(self, terminator: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> Self

Specify terminator to use for CSV read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.escape" class="fn">escape</a>(self, escape: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> Self

Specify delimiter to use for CSV read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.newlines_in_values" class="fn">newlines_in_values</a>(self, newlines_in_values: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Specifies whether newlines in (quoted) values are supported.

Parsing newlines in quoted values may be affected by execution behaviour such as parallel file scanning. Setting this to `true` ensures that newlines in values are parsed successfully, which may reduce performance.

The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.file_extension" class="fn">file_extension</a>(self, file_extension: &'a <a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>) -\> Self

Specify the file extension for CSV file selection

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.delimiter_option" class="fn">delimiter_option</a>(self, delimiter: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> Self

Configure delimiter setting with Option, None value will be ignored

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.schema" class="fn">schema</a>(self, schema: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> Self

Specify schema to use for CSV read

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.table_partition_cols" class="fn">table_partition_cols</a>( self, table_partition_cols: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>)\>, ) -\> Self

Specify table_partition_cols for partition pruning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.schema_infer_max_records" class="fn">schema_infer_max_records</a>(self, max_records: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> Self

Configure number of max records to read for schema inference

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.file_compression_type" class="fn">file_compression_type</a>( self, file_compression_type: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/file_compression_type/struct.FileCompressionType.html" class="struct" title="struct datafusion::datasource::file_format::file_compression_type::FileCompressionType">FileCompressionType</a>, ) -\> Self

Configure file compression type

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.file_sort_order" class="fn">file_sort_order</a>(self, file_sort_order: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/logical_expr/struct.SortExpr.html" class="struct" title="struct datafusion::logical_expr::SortExpr">SortExpr</a>\>\>) -\> Self

Configure if file has known sort order

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.null_regex" class="fn">null_regex</a>(self, null_regex: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> Self

Configure the null parsing regex.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#impl-Clone-for-CsvReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions">CsvReadOptions</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions">CsvReadOptions</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#impl-Default-for-CsvReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions">CsvReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#impl-ReadOptions%3C&#39;_%3E-for-CsvReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html" class="trait" title="trait datafusion::datasource::file_format::options::ReadOptions">ReadOptions</a>\<'\_\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::CsvReadOptions">CsvReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.to_listing_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.to_listing_options" class="fn">to_listing_options</a>( &self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, table_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

Helper to convert these user facing options to `ListingTable` options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method.get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.get_resolved_schema" class="fn">get_resolved_schema</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, config: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Infer and resolve the schema from the files/sources provided.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#method._get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#method._get_resolved_schema" class="fn">_get_resolved_schema</a>\<'life0, 'async_trait\>( &'a self, config: &'life0 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'async_trait, 'a: 'async_trait, 'life0: 'async_trait,

helper function to reduce repetitive code. Infers the schema from sources if not provided. Infinite data sources not supported through this function.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.CsvReadOptions.html#blanket-implementations" class="anchor">§</a>
