# Struct CsvReader Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/reader.rs.html#33" class="src">Source</a>

``` rust
pub struct CsvReader<R>where
    R: MmapBytesReader,{ /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Create a new DataFrame by reading a csv file.

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_io::prelude::*;
use std::fs::File;

fn example() -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
            .with_has_header(true)
            .try_into_reader_with_file_path(Some("iris.csv".into()))?
            .finish()
}
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#impl-CsvReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#method._with_predicate" class="fn">_with_predicate</a>( self, predicate: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/predicates/trait.PhysicalIoExpr.html" class="trait" title="trait polars_io::predicates::PhysicalIoExpr">PhysicalIoExpr</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#impl-CsvReader%3CR%3E-1" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#method.batched_borrowed" class="fn">batched_borrowed</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.BatchedCsvReader.html" class="struct" title="struct polars::prelude::BatchedCsvReader">BatchedCsvReader</a>\<'\_\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#impl-CsvReader%3CBox%3Cdyn+MmapBytesReader%3E%3E" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#method.batched" class="fn">batched</a>( self, schema: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.OwnedBatchedCsvReader.html" class="struct" title="struct polars::prelude::OwnedBatchedCsvReader">OwnedBatchedCsvReader</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#impl-CsvReader%3CR%3E-2" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#method.with_options" class="fn">with_options</a>(self, options: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

Sets custom CSV read options.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#impl-SerReader%3CR%3E-for-CsvReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html" class="trait" title="trait polars::prelude::SerReader">SerReader</a>\<R\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.new" class="fn">new</a>(reader: R) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html" class="struct" title="struct polars::prelude::CsvReader">CsvReader</a>\<R\>

Create a new CsvReader from a file/stream using default read options. To use non-default read options, first construct [CsvReadOptions](https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html "struct polars::prelude::CsvReadOptions") and then use any of the `(try)_into_` methods.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Read the file and create the DataFrame.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#method.set_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#method.set_rechunk" class="fn">set_rechunk</a>(self, \_rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> Self

where Self: <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html" class="trait" title="trait core::marker::Sized">Sized</a>,

Make sure that all columns are contiguous in memory by aggregating the chunks into a single array.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReader.html#blanket-implementations" class="anchor">§</a>
