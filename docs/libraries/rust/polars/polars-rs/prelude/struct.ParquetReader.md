# Struct ParquetReader Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/read/reader.rs.html#17" class="src">Source</a>

``` rust
pub struct ParquetReader<R>where
    R: Read + Seek,{ /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Read Apache parquet format into a DataFrame.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#impl-ParquetReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.set_low_memory" class="fn">set_low_memory</a>(self, low_memory: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

Try to reduce memory pressure at the expense of performance. If setting this does not reduce memory enough, turn off parallelization.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.read_parallel" class="fn">read_parallel</a>(self, parallel: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ParallelStrategy.html" class="enum" title="enum polars::prelude::ParallelStrategy">ParallelStrategy</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

Read the parquet file in parallel (default). The single threaded reader consumes less memory.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.with_slice" class="fn">with_slice</a>(self, slice: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.with_columns" class="fn">with_columns</a>(self, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

Columns to select/ project

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.with_projection" class="fn">with_projection</a>(self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

Set the reader’s column projection. This counts from 0, meaning that `vec![0, 4]` would select the 1st and 5th column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.with_row_index" class="fn">with_row_index</a>(self, row_index: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

Add a row index column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.with_arrow_schema_projection" class="fn">with_arrow_schema_projection</a>( self, first_schema: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>\>, projected_arrow_schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>\>, allow_missing_columns: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Checks that the file contains all the columns in `projected_arrow_schema` with the same dtype, and sets the projection indices.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.schema" class="fn">schema</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

[`Schema`](https://docs.rs/polars/latest/polars/prelude/type.Schema.html "type polars::prelude::Schema") of the file.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.num_rows" class="fn">num_rows</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Number of rows in the parquet file.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.with_hive_partition_columns" class="fn">with_hive_partition_columns</a>( self, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.with_include_file_path" class="fn">with_include_file_path</a>( self, include_file_path: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>)\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.set_metadata" class="fn">set_metadata</a>(&mut self, metadata: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>\>)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.get_metadata" class="fn">get_metadata</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#impl-SerReader%3CR%3E-for-ParquetReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html" class="trait" title="trait polars::prelude::SerReader">SerReader</a>\<R\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.new" class="fn">new</a>(reader: R) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

Create a new [`ParquetReader`](https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html "struct polars::prelude::ParquetReader") from an existing `Reader`.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.set_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#method.set_rechunk" class="fn">set_rechunk</a>(self, rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html" class="struct" title="struct polars::prelude::ParquetReader">ParquetReader</a>\<R\>

Make sure that all columns are contiguous in memory by aggregating the chunks into a single array.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take the SerReader and return a parsed DataFrame.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetReader.html#blanket-implementations" class="anchor">§</a>
