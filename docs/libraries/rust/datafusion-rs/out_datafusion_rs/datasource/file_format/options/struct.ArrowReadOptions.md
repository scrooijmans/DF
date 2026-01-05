# Struct ArrowReadOptions Copy item path

<a href="https://docs.rs/datafusion/50.2.0/src/datafusion/datasource/file_format/options.rs.html#337-347" class="src">Source</a>

``` rust
pub struct ArrowReadOptions<'a> {
    pub schema: Option<&'a Schema>,
    pub file_extension: &'a str,
    pub table_partition_cols: Vec<(String, DataType)>,
}
```

Expand description

Options that control the reading of ARROW files.

Note this structure is supplied when a datasource is created and can not not vary from statement to statement. For settings that can vary statement to statement see [`ConfigOptions`](https://docs.rs/datafusion/50.2.0/datafusion/config/struct.ConfigOptions.html "struct datafusion::config::ConfigOptions").

## Fields<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<&'a `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema"><code>Schema</code></a>`>`

The data source schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#structfield.file_extension" class="anchor field">§</a>`file_extension: &'a `<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive"><code>str</code></a>

File extension; only files with this extension are selected for data input. Defaults to `FileType::ARROW.get_ext().as_str()`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#structfield.table_partition_cols" class="anchor field">§</a>`table_partition_cols: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<(`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`, `<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType"><code>DataType</code></a>`)>`

Partition Columns

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#impl-ArrowReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ArrowReadOptions">ArrowReadOptions</a>\<'a\>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method.table_partition_cols" class="fn">table_partition_cols</a>( self, table_partition_cols: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/enum.DataType.html" class="enum" title="enum datafusion::common::arrow::datatypes::DataType">DataType</a>)\>, ) -\> Self

Specify table_partition_cols for partition pruning

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method.schema" class="fn">schema</a>(self, schema: &'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>) -\> Self

Specify schema to use for AVRO read

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#impl-Clone-for-ArrowReadOptions%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ArrowReadOptions">ArrowReadOptions</a>\<'a\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ArrowReadOptions">ArrowReadOptions</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#impl-Default-for-ArrowReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ArrowReadOptions">ArrowReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#impl-ReadOptions%3C&#39;_%3E-for-ArrowReadOptions%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html" class="trait" title="trait datafusion::datasource::file_format::options::ReadOptions">ReadOptions</a>\<'\_\> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html" class="struct" title="struct datafusion::datasource::file_format::options::ArrowReadOptions">ArrowReadOptions</a>\<'\_\>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method.to_listing_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.to_listing_options" class="fn">to_listing_options</a>( &self, config: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, \_table_options: <a href="https://docs.rs/datafusion/50.2.0/datafusion/config/struct.TableOptions.html" class="struct" title="struct datafusion::config::TableOptions">TableOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingOptions.html" class="struct" title="struct datafusion::datasource::listing::ListingOptions">ListingOptions</a>

Helper to convert these user facing options to `ListingTable` options

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method.get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#tymethod.get_resolved_schema" class="fn">get_resolved_schema</a>\<'life0, 'life1, 'async_trait\>( &'life0 self, config: &'life1 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: 'async_trait, 'life0: 'async_trait, 'life1: 'async_trait,

Infer and resolve the schema from the files/sources provided.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#method._get_resolved_schema" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/trait.ReadOptions.html#method._get_resolved_schema" class="fn">_get_resolved_schema</a>\<'life0, 'async_trait\>( &'a self, config: &'life0 <a href="https://docs.rs/datafusion/50.2.0/datafusion/prelude/struct.SessionConfig.html" class="struct" title="struct datafusion::prelude::SessionConfig">SessionConfig</a>, state: <a href="https://docs.rs/datafusion/50.2.0/datafusion/execution/session_state/struct.SessionState.html" class="struct" title="struct datafusion::execution::session_state::SessionState">SessionState</a>, table_path: <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/listing/struct.ListingTableUrl.html" class="struct" title="struct datafusion::datasource::listing::ListingTableUrl">ListingTableUrl</a>, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&'a <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html" class="struct" title="struct core::pin::Pin">Pin</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html" class="trait" title="trait core::future::future::Future">Future</a>\<Output = <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/type.Result.html" class="type" title="type datafusion::error::Result">Result</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/type.SchemaRef.html" class="type" title="type datafusion::common::arrow::datatypes::SchemaRef">SchemaRef</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> + 'async_trait\>\>

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> + 'async_trait, 'a: 'async_trait, 'life0: 'async_trait,

helper function to reduce repetitive code. Infers the schema from sources if not provided. Infinite data sources not supported through this function.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/options/struct.ArrowReadOptions.html#blanket-implementations" class="anchor">§</a>
