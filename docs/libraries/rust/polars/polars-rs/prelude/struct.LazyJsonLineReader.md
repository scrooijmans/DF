# Struct LazyJsonLineReader Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/scan/ndjson.rs.html#18" class="src">Source</a>

``` rust
pub struct LazyJsonLineReader { /* private fields */ }
```

Available on **crate feature `lazy`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#impl-LazyJsonLineReader" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.new_paths" class="fn">new_paths</a>(paths: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.new_with_sources" class="fn">new_with_sources</a>(sources: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.new" class="fn">new</a>(path: <a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_row_index" class="fn">with_row_index</a>(self, row_index: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Add a row index column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_ignore_errors" class="fn">with_ignore_errors</a>(self, ignore_errors: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Set values as `Null` if parsing fails because of schema mismatches.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_n_rows" class="fn">with_n_rows</a>(self, num_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Try to stop parsing when `n` rows are parsed. During multithreaded parsing the upper bound `n` cannot be guaranteed.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_infer_schema_length" class="fn">with_infer_schema_length</a>( self, num_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Set the number of rows to use when inferring the json schema. the default is 100 rows. Ignored when the schema is specified explicitly using [`Self::with_schema`](https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_schema "method polars::prelude::LazyJsonLineReader::with_schema"). Setting to `None` will do a full table scan, very slow.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_schema" class="fn">with_schema</a>( self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Set the JSON file’s schema

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_schema_overwrite" class="fn">with_schema_overwrite</a>( self, schema_overwrite: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Set the JSON file’s schema

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.low_memory" class="fn">low_memory</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Reduce memory usage at the expense of performance

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_batch_size" class="fn">with_batch_size</a>( self, batch_size: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/core/num/nonzero/struct.NonZero.html" class="struct" title="struct core::num::nonzero::NonZero">NonZero</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_cloud_options" class="fn">with_cloud_options</a>( self, cloud_options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_include_file_paths" class="fn">with_include_file_paths</a>( self, include_file_paths: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#impl-Clone-for-LazyJsonLineReader" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#impl-LazyFileListReader-for-LazyJsonLineReader" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html" class="trait" title="trait polars::prelude::LazyFileListReader">LazyFileListReader</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_rechunk" class="fn">with_rechunk</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Rechunk the memory to contiguous chunks when parsing is done.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.n_rows" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.n_rows" class="fn">n_rows</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Try to stop parsing when `n` rows are parsed. During multithreaded parsing the upper bound `n` cannot be guaranteed.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.row_index" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.row_index" class="fn">row_index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>

Add a row index column.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.cloud_options" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.cloud_options" class="fn">cloud_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>

[CloudOptions](https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html "struct polars::prelude::cloud::CloudOptions") used to list files.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the final [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame").

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.finish_no_glob" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.finish_no_glob" class="fn">finish_no_glob</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the final [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame"). This method assumes, that path is *not* a glob. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.finish_no_glob)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.sources" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.sources" class="fn">sources</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

Get the sources for this reader.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_sources" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_sources" class="fn">with_sources</a>(self, sources: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Set sources of the scanned files.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_n_rows-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_n_rows" class="fn">with_n_rows</a>(self, n_rows: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Configure the row limit.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_row_index-1" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_row_index" class="fn">with_row_index</a>( self, row_index: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>

Configure the row index.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.rechunk" class="fn">rechunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Rechunk the memory to contiguous chunks when parsing is done.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.concat_impl" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.concat_impl" class="fn">concat_impl</a>(&self, lfs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Recommended concatenation of [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame")s from many input files. [Read more](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.concat_impl)

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.glob" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.glob" class="fn">glob</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#method.with_paths" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.with_paths" class="fn">with_paths</a>(self, paths: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>) -\> Self

Set paths of the scanned files.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html#blanket-implementations" class="anchor">§</a>
