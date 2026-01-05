# Struct LazyCsvReader Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/scan/csv.rs.html#18" class="src">Source</a>

``` rust
pub struct LazyCsvReader { /* private fields */ }
```

Available on **crate feature `lazy`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#impl-LazyCsvReader" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.map_parse_options" class="fn">map_parse_options</a>\<F\>(self, map_func: F) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvParseOptions.html" class="struct" title="struct polars::prelude::CsvParseOptions">CsvParseOptions</a>,

Re-export to shorten code.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.new_paths" class="fn">new_paths</a>(paths: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.new_with_sources" class="fn">new_with_sources</a>(sources: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.new" class="fn">new</a>(path: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_skip_rows_after_header" class="fn">with_skip_rows_after_header</a>(self, offset: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Skip this number of rows after the header location.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_row_index" class="fn">with_row_index</a>(self, row_index: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Add a row index column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_n_rows" class="fn">with_n_rows</a>(self, num_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Try to stop parsing when `n` rows are parsed. During multithreaded parsing the upper bound `n` cannot be guaranteed.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_infer_schema_length" class="fn">with_infer_schema_length</a>(self, num_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set the number of rows to use when inferring the csv schema. The default is 100 rows. Setting to [None](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") will do a full table scan, which is very slow.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_ignore_errors" class="fn">with_ignore_errors</a>(self, ignore: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Continue with next batch when a ParserError is encountered.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_schema" class="fn">with_schema</a>(self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set the CSV file’s schema

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_skip_rows" class="fn">with_skip_rows</a>(self, skip_rows: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Skip the first `n` rows during parsing. The header will be parsed at row `n`. Note that by row we mean valid CSV, encoding and comments are respected.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_skip_lines" class="fn">with_skip_lines</a>(self, skip_lines: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Skip the first `n` lines during parsing. The header will be parsed at line `n`. We don’t respect CSV escaping when skipping lines.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_dtype_overwrite" class="fn">with_dtype_overwrite</a>( self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Overwrite the schema with the dtypes in this given Schema. The given schema may be a subset of the total schema.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_has_header" class="fn">with_has_header</a>(self, has_header: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set whether the CSV file has headers

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_chunk_size" class="fn">with_chunk_size</a>(self, chunk_size: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Sets the chunk size used by the parser. This influences performance. This can be used as a way to reduce memory usage during the parsing at the cost of performance.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_separator" class="fn">with_separator</a>(self, separator: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set the CSV file’s column separator as a byte character

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_comment_prefix" class="fn">with_comment_prefix</a>( self, comment_prefix: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set the comment prefix for this instance. Lines starting with this prefix will be ignored.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_quote_char" class="fn">with_quote_char</a>(self, quote_char: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set the `char` used as quote char. The default is `b'"'`. If set to [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None") quoting is disabled.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_eol_char" class="fn">with_eol_char</a>(self, eol_char: <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set the `char` used as end of line. The default is `b'\n'`.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_null_values" class="fn">with_null_values</a>(self, null_values: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.NullValues.html" class="enum" title="enum polars::prelude::NullValues">NullValues</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set values that will be interpreted as missing/ null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_missing_is_null" class="fn">with_missing_is_null</a>(self, missing_is_null: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Treat missing fields as null.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_cache" class="fn">with_cache</a>(self, cache: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Cache the DataFrame after reading.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_low_memory" class="fn">with_low_memory</a>(self, low_memory: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Reduce memory usage at the expense of performance

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_encoding" class="fn">with_encoding</a>(self, encoding: <a href="https://docs.rs/polars/latest/polars/prelude/enum.CsvEncoding.html" class="enum" title="enum polars::prelude::CsvEncoding">CsvEncoding</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set [`CsvEncoding`](https://docs.rs/polars/latest/polars/prelude/enum.CsvEncoding.html "enum polars::prelude::CsvEncoding")

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_try_parse_dates" class="fn">with_try_parse_dates</a>(self, try_parse_dates: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Automatically try to parse dates/datetimes and time. If parsing fails, columns remain of dtype [`DataType::String`](https://docs.rs/polars/latest/polars/prelude/enum.DataType.html#variant.String "variant polars::prelude::DataType::String").

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_raise_if_empty" class="fn">with_raise_if_empty</a>(self, raise_if_empty: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Raise an error if CSV is empty (otherwise return an empty frame)

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_truncate_ragged_lines" class="fn">with_truncate_ragged_lines</a>( self, truncate_ragged_lines: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Truncate lines that are longer than the schema.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_decimal_comma" class="fn">with_decimal_comma</a>(self, decimal_comma: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_glob" class="fn">with_glob</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Expand path given via globbing rules.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_cloud_options" class="fn">with_cloud_options</a>( self, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_schema_modify" class="fn">with_schema_modify</a>\<F\>(self, f: F) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

where F: <a href="https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html" class="trait" title="trait core::ops::function::Fn">Fn</a>(<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>,

Modify a schema before we run the lazy scanning.

Important! Run this function latest in the builder!

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_include_file_paths" class="fn">with_include_file_paths</a>( self, include_file_paths: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#impl-Clone-for-LazyCsvReader" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#impl-LazyFileListReader-for-LazyCsvReader" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html" class="trait" title="trait polars::prelude::LazyFileListReader">LazyFileListReader</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the final [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_rechunk" class="fn">with_rechunk</a>(self, rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Rechunk the memory to contiguous chunks when parsing is done.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.n_rows" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.n_rows" class="fn">n_rows</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Try to stop parsing when `n` rows are parsed. During multithreaded parsing the upper bound `n` cannot be guaranteed.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.row_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.row_index" class="fn">row_index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>

Return the row index settings.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.cloud_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.cloud_options" class="fn">cloud_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>

[CloudOptions](https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html "struct polars::prelude::cloud::CloudOptions") used to list files.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.finish_no_glob" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.finish_no_glob" class="fn">finish_no_glob</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the final [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame"). This method assumes, that path is *not* a glob. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.finish_no_glob)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.glob" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.glob" class="fn">glob</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.sources" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.sources" class="fn">sources</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

Get the sources for this reader.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_sources" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_sources" class="fn">with_sources</a>(self, sources: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Set sources of the scanned files.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_n_rows-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_n_rows" class="fn">with_n_rows</a>(self, n_rows: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Configure the row limit.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_row_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_row_index" class="fn">with_row_index</a>(self, row_index: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

Configure the row index.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.rechunk" class="fn">rechunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Rechunk the memory to contiguous chunks when parsing is done.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.concat_impl" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.concat_impl" class="fn">concat_impl</a>(&self, lfs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Recommended concatenation of [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame")s from many input files. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.concat_impl)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#method.with_paths" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.with_paths" class="fn">with_paths</a>(self, paths: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>) -\> Self

Set paths of the scanned files.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html#blanket-implementations" class="anchor">§</a>
