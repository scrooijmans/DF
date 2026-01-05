# Struct FileMetadata Copy item path

<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/src/polars_parquet/parquet/metadata/file_metadata.rs.html#14" class="src">Source</a>

``` rust
pub struct FileMetadata {
    pub version: i32,
    pub num_rows: usize,
    pub max_row_group_height: usize,
    pub created_by: Option<String>,
    pub row_groups: Vec<RowGroupMetadata>,
    pub key_value_metadata: Option<Vec<KeyValue>>,
    pub schema_descr: SchemaDescriptor,
    pub column_orders: Option<Vec<ColumnOrder>>,
}
```

Available on **crate feature `polars-io`** only.

Expand description

Metadata for a Parquet file.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.version" class="anchor field">§</a>`version: `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>

version of this file.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.num_rows" class="anchor field">§</a>`num_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

number of rows in the file.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.max_row_group_height" class="anchor field">§</a>`max_row_group_height: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Max row group height, useful for sharing column materializations.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.created_by" class="anchor field">§</a>`created_by: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String"><code>String</code></a>`>`

String message for application that wrote this file.

This should have the following format: `<application> version <application version> (build <application build hash>)`.

``` shell
parquet-mr version 1.8.0 (build 0fda28af84b9746396014ad6a415b90592a98b3b)
```

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.row_groups" class="anchor field">§</a>`row_groups: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/metadata/row_metadata/struct.RowGroupMetadata.html" class="struct" title="struct polars_parquet::parquet::metadata::row_metadata::RowGroupMetadata"><code>RowGroupMetadata</code></a>`>`

The row groups of this file

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.key_value_metadata" class="anchor field">§</a>`key_value_metadata: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars-parquet-format/0.1.0/x86_64-unknown-linux-gnu/polars_parquet_format/parquet_format/struct.KeyValue.html" class="struct" title="struct polars_parquet_format::parquet_format::KeyValue"><code>KeyValue</code></a>`>>`

key_value_metadata of this file.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.schema_descr" class="anchor field">§</a>`schema_descr: `<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/metadata/schema_descriptor/struct.SchemaDescriptor.html" class="struct" title="struct polars_parquet::parquet::metadata::schema_descriptor::SchemaDescriptor"><code>SchemaDescriptor</code></a>

schema descriptor.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#structfield.column_orders" class="anchor field">§</a>`column_orders: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/metadata/column_order/enum.ColumnOrder.html" class="enum" title="enum polars_parquet::parquet::metadata::column_order::ColumnOrder"><code>ColumnOrder</code></a>`>>`

Column (sort) order used for `min` and `max` values of each column in this file.

Each column order corresponds to one column, determined by its position in the list, matching the position of the column in the schema.

When `None` is returned, there are no column orders available, and each column should be assumed to have undefined (legacy) column order.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#impl-FileMetadata" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#method.schema" class="fn">schema</a>(&self) -\> &<a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/metadata/schema_descriptor/struct.SchemaDescriptor.html" class="struct" title="struct polars_parquet::parquet::metadata::schema_descriptor::SchemaDescriptor">SchemaDescriptor</a>

Returns the [`SchemaDescriptor`](https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/metadata/schema_descriptor/struct.SchemaDescriptor.html "struct polars_parquet::parquet::metadata::schema_descriptor::SchemaDescriptor") that describes schema of this file.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#method.key_value_metadata" class="fn">key_value_metadata</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars-parquet-format/0.1.0/x86_64-unknown-linux-gnu/polars_parquet_format/parquet_format/struct.KeyValue.html" class="struct" title="struct polars_parquet_format::parquet_format::KeyValue">KeyValue</a>\>\>

returns the metadata

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#method.column_order" class="fn">column_order</a>(&self, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/metadata/column_order/enum.ColumnOrder.html" class="enum" title="enum polars_parquet::parquet::metadata::column_order::ColumnOrder">ColumnOrder</a>

Returns column order for `i`th column in this file. If column orders are not available, returns undefined (legacy) column order.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#method.try_from_thrift" class="fn">try_from_thrift</a>( metadata: <a href="https://docs.rs/polars-parquet-format/0.1.0/x86_64-unknown-linux-gnu/polars_parquet_format/parquet_format/struct.FileMetaData.html" class="struct" title="struct polars_parquet_format::parquet_format::FileMetaData">FileMetaData</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>, <a href="https://docs.rs/polars-parquet/0.51.0/x86_64-unknown-linux-gnu/polars_parquet/parquet/error/enum.ParquetError.html" class="enum" title="enum polars_parquet::parquet::error::ParquetError">ParquetError</a>\>

Deserializes \[`crate::parquet::thrift_format::FileMetadata`\] into this struct

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#impl-Clone-for-FileMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#impl-Debug-for-FileMetadata" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html#blanket-implementations" class="anchor">§</a>
