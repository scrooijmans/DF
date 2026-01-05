# Struct DataFileBuilder Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/data_file.rs.html#33" class="src">Source</a>

``` rust
pub struct DataFileBuilder { /* private fields */ }
```

Expand description

Builder for [`DataFile`](https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html).

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#impl-DataFileBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html" class="struct" title="struct iceberg::spec::DataFileBuilder">DataFileBuilder</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.content" class="fn">content</a>(&mut self, value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataContentType.html" class="enum" title="enum iceberg::spec::DataContentType">DataContentType</a>) -\> &mut Self

field id: 134

Type of content stored by the data file: data, equality deletes, or position deletes (all v1 files are data files)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.file_path" class="fn">file_path</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>) -\> &mut Self

field id: 100

Full URI for the file with FS scheme

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.file_format" class="fn">file_format</a>(&mut self, value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataFileFormat.html" class="enum" title="enum iceberg::spec::DataFileFormat">DataFileFormat</a>) -\> &mut Self

field id: 101

String file format name, `avro`, `orc`, `parquet`, or `puffin`

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.partition" class="fn">partition</a>(&mut self, value: <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Struct.html" class="struct" title="struct iceberg::spec::Struct">Struct</a>) -\> &mut Self

field id: 102

Partition data tuple, schema based on the partition spec output using partition field ids for the struct field ids

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.record_count" class="fn">record_count</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> &mut Self

field id: 103

Number of records in this file, or the cardinality of a deletion vector

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.file_size_in_bytes" class="fn">file_size_in_bytes</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>) -\> &mut Self

field id: 104

Total file size in bytes

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.column_sizes" class="fn">column_sizes</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> &mut Self

field id: 108 key field id: 117 value field id: 118

Map from column id to the total size on disk of all regions that store the column. Does not include bytes necessary to read other columns, like footers. Leave null for row-oriented formats (Avro)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.value_counts" class="fn">value_counts</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> &mut Self

field id: 109 key field id: 119 value field id: 120

Map from column id to number of values in the column (including null and NaN values)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.null_value_counts" class="fn">null_value_counts</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> &mut Self

field id: 110 key field id: 121 value field id: 122

Map from column id to number of null values in the column

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.nan_value_counts" class="fn">nan_value_counts</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>) -\> &mut Self

field id: 137 key field id: 138 value field id: 139

Map from column id to number of NaN values in the column

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.lower_bounds" class="fn">lower_bounds</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>) -\> &mut Self

field id: 125 key field id: 126 value field id: 127

Map from column id to lower bound in the column serialized as binary. Each value must be less than or equal to all non-null, non-NaN values in the column for the file.

Reference:

- [Binary single-value serialization](https://iceberg.apache.org/spec/#binary-single-value-serialization)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.upper_bounds" class="fn">upper_bounds</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>) -\> &mut Self

field id: 128 key field id: 129 value field id: 130

Map from column id to upper bound in the column serialized as binary. Each value must be greater than or equal to all non-null, non-Nan values in the column for the file.

Reference:

- [Binary single-value serialization](https://iceberg.apache.org/spec/#binary-single-value-serialization)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.key_metadata" class="fn">key_metadata</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>\>) -\> &mut Self

field id: 131

Implementation-specific key metadata for encryption

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.split_offsets" class="fn">split_offsets</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> &mut Self

field id: 132 element field id: 133

Split offsets for the data file. For example, all row group offsets in a Parquet file. Must be sorted ascending

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.equality_ids" class="fn">equality_ids</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>) -\> &mut Self

field id: 135 element field id: 136

Field ids used to determine row equality in equality delete files. Required when content is EqualityDeletes and should be null otherwise. Fields with ids listed in this column must be present in the delete file

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.sort_order_id" class="fn">sort_order_id</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> &mut Self

field id: 140

ID representing sort order for this file.

If sort order ID is missing or unknown, then the order is assumed to be unsorted. Only data files and equality delete files should be written with a non-null order id. Position deletes are required to be sorted by file and position, not a table order, and should set sort order id to null. Readers must ignore sort order id for position delete files.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.first_row_id" class="fn">first_row_id</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> &mut Self

field id: 142

The \_row_id for the first row in the data file. For more details, refer to https://github.com/apache/iceberg/blob/main/format/spec.md#first-row-id-inheritance

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.partition_spec_id" class="fn">partition_spec_id</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>) -\> &mut Self

This field is not included in spec. It is just store in memory representation used in process.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.referenced_data_file" class="fn">referenced_data_file</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>) -\> &mut Self

field id: 143

Fully qualified location (URI with FS scheme) of a data file that all deletes reference. Position delete metadata can use `referenced_data_file` when all deletes tracked by the entry are in a single data file. Setting the referenced file is required for deletion vectors.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.content_offset" class="fn">content_offset</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> &mut Self

field: 144

The offset in the file where the content starts. The `content_offset` and `content_size_in_bytes` fields are used to reference a specific blob for direct access to a deletion vector. For deletion vectors, these values are required and must exactly match the `offset` and `length` stored in the Puffin footer for the deletion vector blob.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.content_size_in_bytes" class="fn">content_size_in_bytes</a>(&mut self, value: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>) -\> &mut Self

field: 145

The length of a referenced content stored in the file; required if `content_offset` is present

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.build" class="fn">build</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataFileBuilderError.html" class="enum" title="enum iceberg::spec::DataFileBuilderError">DataFileBuilderError</a>\>

Builds a new `DataFile`.

##### <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#errors" class="doc-anchor">§</a>Errors

If a required field has not been initialized.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#impl-Clone-for-DataFileBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html" class="struct" title="struct iceberg::spec::DataFileBuilder">DataFileBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html" class="struct" title="struct iceberg::spec::DataFileBuilder">DataFileBuilder</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#impl-Default-for-DataFileBuilder" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html" class="struct" title="struct iceberg::spec::DataFileBuilder">DataFileBuilder</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> Self

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFileBuilder.html#blanket-implementations" class="anchor">§</a>
