# Struct DataFile Copy item path

<a href="https://docs.rs/iceberg/0.7.0/src/iceberg/spec/manifest/data_file.rs.html#34-180" class="src">Source</a>

``` rust
pub struct DataFile { /* private fields */ }
```

Expand description

Data file carries data file path, partition tuple, metrics, …

## Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#impl-DataFile" class="anchor">§</a>

### impl <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.content_type" class="fn">content_type</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataContentType.html" class="enum" title="enum iceberg::spec::DataContentType">DataContentType</a>

Get the content type of the data file (data, equality deletes, or position deletes)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.file_path" class="fn">file_path</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>

Get the file path as full URI with FS scheme

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.file_format" class="fn">file_format</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/enum.DataFileFormat.html" class="enum" title="enum iceberg::spec::DataFileFormat">DataFileFormat</a>

Get the file format of the file (avro, orc or parquet).

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.partition" class="fn">partition</a>(&self) -\> &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Struct.html" class="struct" title="struct iceberg::spec::Struct">Struct</a>

Get the partition values of the file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.record_count" class="fn">record_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Get the record count in the data file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.file_size_in_bytes" class="fn">file_size_in_bytes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>

Get the file size in bytes.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.column_sizes" class="fn">column_sizes</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Get the column sizes. Map from column id to the total size on disk of all regions that store the column. Does not include bytes necessary to read other columns, like footers. Null for row-oriented formats (Avro)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.value_counts" class="fn">value_counts</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Get the columns value counts for the data file. Map from column id to number of values in the column (including null and NaN values)

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.null_value_counts" class="fn">null_value_counts</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Get the null value counts of the data file. Map from column id to number of null values in the column

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.nan_value_counts" class="fn">nan_value_counts</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>\>

Get the nan value counts of the data file. Map from column id to number of NaN values in the column

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.lower_bounds" class="fn">lower_bounds</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>

Get the lower bounds of the data file values per column. Map from column id to lower bound in the column serialized as binary.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.upper_bounds" class="fn">upper_bounds</a>(&self) -\> &<a href="https://doc.rust-lang.org/nightly/std/collections/hash/map/struct.HashMap.html" class="struct" title="struct std::collections::hash::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>, <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.Datum.html" class="struct" title="struct iceberg::spec::Datum">Datum</a>\>

Get the upper bounds of the data file values per column. Map from column id to upper bound in the column serialized as binary.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.key_metadata" class="fn">key_metadata</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&\[<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\]\>

Get the Implementation-specific key metadata for the data file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.split_offsets" class="fn">split_offsets</a>(&self) -\> &\[<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\]

Get the split offsets of the data file. For example, all row group offsets in a Parquet file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.equality_ids" class="fn">equality_ids</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>\>

Get the equality ids of the data file. Field ids used to determine row equality in equality delete files. null when content is not EqualityDeletes.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.first_row_id" class="fn">first_row_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Get the first row id in the data file.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.sort_order_id" class="fn">sort_order_id</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive">i32</a>\>

Get the sort order id of the data file. Only data files and equality delete files should be written with a non-null order id. Position deletes are required to be sorted by file and position, not a table order, and should set sort order id to null. Readers must ignore sort order id for position delete files.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.referenced_data_file" class="fn">referenced_data_file</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>

Get the fully qualified referenced location for the corresponding data file. Positional delete files could have the field set, and deletion vectors must the field set.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.content_offset" class="fn">content_offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Get the offset in the file where the blob content starts. Only meaningful for puffin blobs, and required for deletion vectors.

#### pub fn <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.content_size_in_bytes" class="fn">content_size_in_bytes</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>

Get the length of a puffin blob. Only meaningful for puffin blobs, and required for deletion vectors.

## Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#impl-Clone-for-DataFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#impl-Debug-for-DataFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/fmt/type.Result.html" class="type" title="type core::fmt::Result">Result</a>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#impl-PartialEq-for-DataFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#impl-Eq-for-DataFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#impl-StructuralPartialEq-for-DataFile" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html" class="struct" title="struct iceberg::spec::DataFile">DataFile</a>

## Auto Trait Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/iceberg/0.7.0/iceberg/spec/struct.DataFile.html#blanket-implementations" class="anchor">§</a>
