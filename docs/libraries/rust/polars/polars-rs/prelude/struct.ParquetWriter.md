# Struct ParquetWriter Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/write/writer.rs.html#35" class="src">Source</a>

``` rust
pub struct ParquetWriter<W> { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Write a DataFrame to Parquet format.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#impl-ParquetWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.new" class="fn">new</a>(writer: W) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

Create a new writer

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.with_compression" class="fn">with_compression</a>( self, compression: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ParquetCompression.html" class="enum" title="enum polars::prelude::ParquetCompression">ParquetCompression</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

Set the compression used. Defaults to `Zstd`.

The default compression `Zstd` has very good performance, but may not yet been supported by older readers. If you want more compatibility guarantees, consider using `Snappy`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.with_statistics" class="fn">with_statistics</a>(self, statistics: <a href="https://docs.rs/polars/latest/polars/prelude/struct.StatisticsOptions.html" class="struct" title="struct polars::prelude::StatisticsOptions">StatisticsOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

Compute and write statistic

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.with_row_group_size" class="fn">with_row_group_size</a>(self, size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

Set the row group size (in number of rows) during writing. This can reduce memory pressure and improve writing performance.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.with_data_page_size" class="fn">with_data_page_size</a>(self, limit: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

Sets the maximum bytes size of a data page. If `None` will be 1024^2 bytes.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.set_parallel" class="fn">set_parallel</a>(self, parallel: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

Serialize columns in parallel

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.with_key_value_metadata" class="fn">with_key_value_metadata</a>( self, key_value_metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.KeyValueMetadata.html" class="enum" title="enum polars::prelude::KeyValueMetadata">KeyValueMetadata</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

Set custom file-level key value metadata for the Parquet file

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.with_context_info" class="fn">with_context_info</a>( self, context_info: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/hashbrown/0.15.4/x86_64-unknown-linux-gnu/hashbrown/map/struct.HashMap.html" class="struct" title="struct hashbrown::map::HashMap">HashMap</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>, <a href="https://docs.rs/foldhash/0.1.5/x86_64-unknown-linux-gnu/foldhash/quality/struct.RandomState.html" class="struct" title="struct foldhash::quality::RandomState">RandomState</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html" class="struct" title="struct polars::prelude::ParquetWriter">ParquetWriter</a>\<W\>

Set context information for the writer

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.batched" class="fn">batched</a>( self, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/parquet/write/batched_writer/struct.BatchedWriter.html" class="struct" title="struct polars_io::parquet::write::batched_writer::BatchedWriter">BatchedWriter</a>\<W\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#method.finish" class="fn">finish</a>(self, df: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u64.html" class="primitive">u64</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Write the given DataFrame in the writer `W`. Returns the total size of the file.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetWriter.html#blanket-implementations" class="anchor">§</a>
