# Struct IpcReader Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ipc/ipc_file.rs.html#81" class="src">Source</a>

``` rust
pub struct IpcReader<R>where
    R: MmapBytesReader,{ /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Read Arrows IPC format into a DataFrame

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use std::fs::File;
use polars_io::ipc::IpcReader;
use polars_io::SerReader;

fn example() -> PolarsResult<DataFrame> {
    let file = File::open("file.ipc").expect("file not found");

    IpcReader::new(file)
        .finish()
}
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#impl-IpcReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.schema" class="fn">schema</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get arrow schema of the Ipc File.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.custom_metadata" class="fn">custom_metadata</a>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get schema-level custom metadata of the Ipc file

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.with_n_rows" class="fn">with_n_rows</a>(self, num_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

Stop reading when `n` rows are read.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.with_columns" class="fn">with_columns</a>(self, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

Columns to select/ project

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.with_hive_partition_columns" class="fn">with_hive_partition_columns</a>( self, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.with_include_file_path" class="fn">with_include_file_path</a>( self, include_file_path: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>)\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.with_row_index" class="fn">with_row_index</a>(self, row_index: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

Add a row index column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.with_projection" class="fn">with_projection</a>(self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

Set the reader’s column projection. This counts from 0, meaning that `vec![0, 4]` would select the 1st and 5th column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.memory_mapped" class="fn">memory_mapped</a>(self, path_buf: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/path/struct.PathBuf.html" class="struct" title="struct std::path::PathBuf">PathBuf</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

Set if the file is to be memory_mapped. Only works with uncompressed files. The file name must be passed to register the memory mapped file.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.finish_with_scan_ops" class="fn">finish_with_scan_ops</a>( self, predicate: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<dyn <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/predicates/trait.PhysicalIoExpr.html" class="trait" title="trait polars_io::predicates::PhysicalIoExpr">PhysicalIoExpr</a>\>\>, verbose: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#impl-SerReader%3CR%3E-for-IpcReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html" class="trait" title="trait polars::prelude::SerReader">SerReader</a>\<R\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

where R: <a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/trait.MmapBytesReader.html" class="trait" title="trait polars_io::mmap::MmapBytesReader">MmapBytesReader</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.new" class="fn">new</a>(reader: R) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

Create a new instance of the [`SerReader`](https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html "trait polars::prelude::SerReader")

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.set_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#method.set_rechunk" class="fn">set_rechunk</a>(self, rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html" class="struct" title="struct polars::prelude::IpcReader">IpcReader</a>\<R\>

Make sure that all columns are contiguous in memory by aggregating the chunks into a single array.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take the SerReader and return a parsed DataFrame.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcReader.html#blanket-implementations" class="anchor">§</a>
