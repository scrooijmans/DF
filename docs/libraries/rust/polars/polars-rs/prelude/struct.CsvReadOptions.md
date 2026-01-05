# Struct CsvReadOptions Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/options.rs.html#17" class="src">Source</a>

``` rust
pub struct CsvReadOptions {Show 21 fields
    pub path: Option<PathBuf>,
    pub rechunk: bool,
    pub n_threads: Option<usize>,
    pub low_memory: bool,
    pub n_rows: Option<usize>,
    pub row_index: Option<RowIndex>,
    pub columns: Option<Arc<[PlSmallStr]>>,
    pub projection: Option<Arc<Vec<usize>>>,
    pub schema: Option<Arc<Schema<DataType>>>,
    pub schema_overwrite: Option<Arc<Schema<DataType>>>,
    pub dtype_overwrite: Option<Arc<Vec<DataType>>>,
    pub parse_options: Arc<CsvParseOptions>,
    pub has_header: bool,
    pub chunk_size: usize,
    pub skip_rows: usize,
    pub skip_lines: usize,
    pub skip_rows_after_header: usize,
    pub infer_schema_length: Option<usize>,
    pub raise_if_empty: bool,
    pub ignore_errors: bool,
    pub fields_to_cast: Vec<Field>,
}
```

Available on **crate feature `polars-io`** only.

## Fields<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#fields" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.path" class="anchor field">§</a>`path: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf"><code>PathBuf</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.rechunk" class="anchor field">§</a>`rechunk: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.n_threads" class="anchor field">§</a>`n_threads: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.low_memory" class="anchor field">§</a>`low_memory: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.n_rows" class="anchor field">§</a>`n_rows: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.row_index" class="anchor field">§</a>`row_index: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex"><code>RowIndex</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.columns" class="anchor field">§</a>`columns: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<[`<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr"><code>PlSmallStr</code></a>`]>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.projection" class="anchor field">§</a>`projection: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.schema" class="anchor field">§</a>`schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.schema_overwrite" class="anchor field">§</a>`schema_overwrite: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema"><code>Schema</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.dtype_overwrite" class="anchor field">§</a>`dtype_overwrite: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType"><code>DataType</code></a>`>>>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.parse_options" class="anchor field">§</a>`parse_options: `<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc"><code>Arc</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions"><code>CsvParseOptions</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.has_header" class="anchor field">§</a>`has_header: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.chunk_size" class="anchor field">§</a>`chunk_size: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.skip_rows" class="anchor field">§</a>`skip_rows: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Skip rows according to the CSV spec.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.skip_lines" class="anchor field">§</a>`skip_lines: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>

Skip lines according to newline char (e.g. escaping will be ignored)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.skip_rows_after_header" class="anchor field">§</a>`skip_rows_after_header: `<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.infer_schema_length" class="anchor field">§</a>`infer_schema_length: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<`<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive"><code>usize</code></a>`>`<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.raise_if_empty" class="anchor field">§</a>`raise_if_empty: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.ignore_errors" class="anchor field">§</a>`ignore_errors: `<a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive"><code>bool</code></a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#structfield.fields_to_cast" class="anchor field">§</a>`fields_to_cast: `<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec"><code>Vec</code></a>`<`<a href="https://docs.rs/polars/latest/polars/prelude/struct.Field.html" class="struct" title="struct polars::prelude::Field"><code>Field</code></a>`>`

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.get_parse_options" class="fn">get_parse_options</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_path" class="fn">with_path</a>\<P\>(self, path: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<P\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

where P: <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_rechunk" class="fn">with_rechunk</a>(self, rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Whether to makes the columns contiguous in memory.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_n_threads" class="fn">with_n_threads</a>(self, n_threads: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Number of threads to use for reading. Defaults to the size of the polars thread pool.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_low_memory" class="fn">with_low_memory</a>(self, low_memory: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Reduce memory consumption at the expense of performance

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_n_rows" class="fn">with_n_rows</a>(self, n_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Limits the number of rows to read.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_row_index" class="fn">with_row_index</a>(self, row_index: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Adds a row index column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_columns" class="fn">with_columns</a>(self, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\]\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Which columns to select.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_projection" class="fn">with_projection</a>( self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Which columns to select denoted by their index. The index starts from 0 (i.e. \[0, 4\] would select the 1st and 5th column).

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_schema" class="fn">with_schema</a>( self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Set the schema to use for CSV file. The length of the schema must match the number of columns in the file. If this is [None](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), the schema is inferred from the file.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_schema_overwrite" class="fn">with_schema_overwrite</a>( self, schema_overwrite: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Overwrites the data types in the schema by column name.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_dtype_overwrite" class="fn">with_dtype_overwrite</a>( self, dtype_overwrite: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Overwrite the dtypes in the schema in the order of the slice that’s given. This is useful if you don’t know the column names beforehand

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_parse_options" class="fn">with_parse_options</a>( self, parse_options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Sets the CSV parsing options. See [map_parse_options](https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.map_parse_options "method polars::prelude::CsvReadOptions::map_parse_options") for an easier way to mutate them in-place.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_has_header" class="fn">with_has_header</a>(self, has_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Sets whether the CSV file has a header row.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_chunk_size" class="fn">with_chunk_size</a>(self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Sets the chunk size used by the parser. This influences performance.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_skip_rows" class="fn">with_skip_rows</a>(self, skip_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Start reading after `skip_rows` rows. The header will be parsed at this offset. Note that we respect CSV escaping/comments when skipping rows. If you want to skip by newline char only, use `skip_lines`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_skip_lines" class="fn">with_skip_lines</a>(self, skip_lines: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Start reading after `skip_lines` lines. The header will be parsed at this offset. Note that CSV escaping will not be respected when skipping lines. If you want to skip valid CSV rows, use `skip_rows`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_skip_rows_after_header" class="fn">with_skip_rows_after_header</a>( self, skip_rows_after_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Number of rows to skip after the header row.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_infer_schema_length" class="fn">with_infer_schema_length</a>( self, infer_schema_length: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Set the number of rows to use when inferring the csv schema. The default is 100 rows. Setting to [None](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") will do a full table scan, which is very slow.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_raise_if_empty" class="fn">with_raise_if_empty</a>(self, raise_if_empty: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Whether to raise an error if the frame is empty. By default an empty DataFrame is returned.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.with_ignore_errors" class="fn">with_ignore_errors</a>(self, ignore_errors: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Continue with next batch when a ParserError is encountered.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.map_parse_options" class="fn">map_parse_options</a>\<F\>(self, map_func: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>,

Apply a function to the parse options.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-CsvReadOptions-1" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.try_into_reader_with_file_path" class="fn">try_into_reader_with_file_path</a>( self, path: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<<a href="https://doc.rust-lang.org/nightly/std/fs/struct.File.html" class="struct" title="struct std::fs::File">File</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Creates a CSV reader using a file path.

##### <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#panics" class="doc-anchor">§</a>Panics

If both self.path and the path parameter are non-null. Only one of them is to be non-null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.into_reader_with_file_handle" class="fn">into_reader_with_file_handle</a>\<R\>(self, reader: R) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

Creates a CSV reader using a file handle.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-CsvReadOptions-2" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.update_with_inference_result" class="fn">update_with_inference_result</a>( &mut self, si_result: &<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>, )

Note: This does not update the schema from the inference result.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-Clone-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-Debug-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-Default-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-Deserialize%3C&#39;de%3E-for-CsvReadOptions" class="anchor">§</a>

### impl\<'de\> <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html" class="trait" title="trait serde::de::Deserialize">Deserialize</a>\<'de\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.deserialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize" class="fn">deserialize</a>\<\_\_D\>( \_\_deserializer: \_\_D, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>, \<\_\_D as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html#associatedtype.Error" class="associatedtype" title="type serde::de::Deserializer::Error">Error</a>\>

where \_\_D: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserializer.html" class="trait" title="trait serde::de::Deserializer">Deserializer</a>\<'de\>,

Deserialize this value from the given Serde deserializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/de/trait.Deserialize.html#tymethod.deserialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-Hash-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html" class="trait" title="trait core::hash::Hash">Hash</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.hash" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash" class="fn">hash</a>\<\_\_H\>(&self, state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut __H</a>)

where \_\_H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>,

Feeds this value into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#tymethod.hash)

1.3.0 · <a href="https://doc.rust-lang.org/nightly/src/core/hash/mod.rs.html#235-237" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.hash_slice" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice" class="fn">hash_slice</a>\<H\>(data: &\[Self\], state: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;mut H</a>)

where H: <a href="https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html" class="trait" title="trait core::hash::Hasher">Hasher</a>, Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Feeds a slice of this type into the given [`Hasher`](https://doc.rust-lang.org/nightly/core/hash/trait.Hasher.html "trait core::hash::Hasher"). [Read more](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html#method.hash_slice)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-PartialEq-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html" class="trait" title="trait core::cmp::PartialEq">PartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.eq" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#tymethod.eq" class="fn">eq</a>(&self, other: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `self` and `other` values to be equal, and is used by `==`.

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/cmp.rs.html#264" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.ne" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html#method.ne" class="fn">ne</a>(&self, other: <a href="https://doc.rust-lang.org/nightly/std/primitive.reference.html" class="primitive">&amp;Rhs</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Tests for `!=`. The default implementation is almost always sufficient, and should not be overridden without very good reason.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-Serialize-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html" class="trait" title="trait serde::ser::Serialize">Serialize</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize" class="fn">serialize</a>\<\_\_S\>( &self, \_\_serializer: \_\_S, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<\<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Ok" class="associatedtype" title="type serde::ser::Serializer::Ok">Ok</a>, \<\_\_S as <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>\>::<a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html#associatedtype.Error" class="associatedtype" title="type serde::ser::Serializer::Error">Error</a>\>

where \_\_S: <a href="https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serializer.html" class="trait" title="trait serde::ser::Serializer">Serializer</a>,

Serialize this value into the given Serde serializer. [Read more](https://docs.rs/serde/1.0.219/x86_64-unknown-linux-gnu/serde/ser/trait.Serialize.html#tymethod.serialize)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-Eq-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html" class="trait" title="trait core::cmp::Eq">Eq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#impl-StructuralPartialEq-for-CsvReadOptions" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.StructuralPartialEq.html" class="trait" title="trait core::marker::StructuralPartialEq">StructuralPartialEq</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html#blanket-implementations" class="anchor">§</a>
