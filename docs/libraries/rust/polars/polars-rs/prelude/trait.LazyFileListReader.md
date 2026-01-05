# Trait LazyFileListReader Copy item path

<a href="https://docs.rs/polars-lazy/0.51.0/x86_64-unknown-linux-gnu/src/polars_lazy/scan/file_list_reader.rs.html#15" class="src">Source</a>

``` rust
pub trait LazyFileListReader: Clone {
Show 14 methods    // Required methods
    fn finish_no_glob(self) -> Result<LazyFrame, PolarsError>;
    fn sources(&self) -> &ScanSources;
    fn with_sources(self, source: ScanSources) -> Self;
    fn with_n_rows(self, n_rows: impl Into<Option<usize>>) -> Self;
    fn with_row_index(self, row_index: impl Into<Option<RowIndex>>) -> Self;
    fn rechunk(&self) -> bool;
    fn with_rechunk(self, toggle: bool) -> Self;
    fn n_rows(&self) -> Option<usize>;
    fn row_index(&self) -> Option<&RowIndex>;

    // Provided methods
    fn finish(self) -> Result<LazyFrame, PolarsError> { ... }
    fn concat_impl(&self, lfs: Vec<LazyFrame>) -> Result<LazyFrame, PolarsError> { ... }
    fn glob(&self) -> bool { ... }
    fn with_paths(self, paths: Arc<[PlPath]>) -> Self { ... }
    fn cloud_options(&self) -> Option<&CloudOptions> { ... }
}
```

Available on **crate feature `lazy`** only.

Expand description

Reads [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame") from a filesystem or a cloud storage. Supports glob patterns.

Use [LazyFileListReader::finish](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.finish "method polars::prelude::LazyFileListReader::finish") to get the final [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame").

## Required Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#required-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.finish_no_glob" class="fn">finish_no_glob</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the final [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame"). This method assumes, that path is *not* a glob.

It is recommended to always use [LazyFileListReader::finish](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.finish "method polars::prelude::LazyFileListReader::finish") method.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.sources" class="fn">sources</a>(&self) -\> &<a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>

Get the sources for this reader.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_sources" class="fn">with_sources</a>(self, source: <a href="https://docs.rs/polars/latest/polars/prelude/enum.ScanSources.html" class="enum" title="enum polars::prelude::ScanSources">ScanSources</a>) -\> Self

Set sources of the scanned files.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_n_rows" class="fn">with_n_rows</a>(self, n_rows: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>) -\> Self

Configure the row limit.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_row_index" class="fn">with_row_index</a>(self, row_index: impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.Into.html" class="trait" title="trait core::convert::Into">Into</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>\>) -\> Self

Configure the row index.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.rechunk" class="fn">rechunk</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Rechunk the memory to contiguous chunks when parsing is done.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.with_rechunk" class="fn">with_rechunk</a>(self, toggle: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

Rechunk the memory to contiguous chunks when parsing is done.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.n_rows" class="fn">n_rows</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Try to stop parsing when `n` rows are parsed. During multithreaded parsing the upper bound `n` cannot be guaranteed.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.row_index" class="fn">row_index</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>

Add a row index column.

## Provided Methods<a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#provided-methods" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get the final [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame").

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.concat_impl" class="fn">concat_impl</a>(&self, lfs: <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html" class="struct" title="struct polars::prelude::LazyFrame">LazyFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Recommended concatenation of [LazyFrame](https://docs.rs/polars/latest/polars/prelude/struct.LazyFrame.html "struct polars::prelude::LazyFrame")s from many input files.

This method should not take into consideration [LazyFileListReader::n_rows](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.n_rows "method polars::prelude::LazyFileListReader::n_rows") nor [LazyFileListReader::row_index](https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#tymethod.row_index "method polars::prelude::LazyFileListReader::row_index").

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.glob" class="fn">glob</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.with_paths" class="fn">with_paths</a>(self, paths: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<\[<a href="https://docs.rs/polars/latest/polars/prelude/enum.PlPath.html" class="enum" title="enum polars::prelude::PlPath">PlPath</a>\]\>) -\> Self

Set paths of the scanned files.

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#method.cloud_options" class="fn">cloud_options</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>

[CloudOptions](https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html "struct polars::prelude::cloud::CloudOptions") used to list files.

## Dyn Compatibility<a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#dyn-compatibility" class="anchor">§</a>

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

*In older versions of Rust, dyn compatibility was called "object safety", so this trait is not object safe.*

## Implementors<a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#implementors" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#impl-LazyFileListReader-for-LazyCsvReader" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html" class="trait" title="trait polars::prelude::LazyFileListReader">LazyFileListReader</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyCsvReader.html" class="struct" title="struct polars::prelude::LazyCsvReader">LazyCsvReader</a>

<a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html#impl-LazyFileListReader-for-LazyJsonLineReader" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/trait.LazyFileListReader.html" class="trait" title="trait polars::prelude::LazyFileListReader">LazyFileListReader</a> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.LazyJsonLineReader.html" class="struct" title="struct polars::prelude::LazyJsonLineReader">LazyJsonLineReader</a>
