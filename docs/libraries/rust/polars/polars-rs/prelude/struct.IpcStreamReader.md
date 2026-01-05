# Struct IpcStreamReader Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ipc/ipc_stream.rs.html#66" class="src">Source</a>

``` rust
pub struct IpcStreamReader<R> { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Read Arrows Stream IPC format into a DataFrame

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use std::fs::File;
use polars_io::ipc::IpcStreamReader;
use polars_io::SerReader;

fn example() -> PolarsResult<DataFrame> {
    let file = File::open("file.ipc").expect("file not found");

    IpcStreamReader::new(file)
        .finish()
}
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#impl-IpcStreamReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.schema" class="fn">schema</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get schema of the Ipc Stream File

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.arrow_schema" class="fn">arrow_schema</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get arrow schema of the Ipc Stream File, this is faster than creating a polars schema.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.custom_metadata" class="fn">custom_metadata</a>( &mut self, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Get schema-level custom metadata of the Ipc Stream file

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.with_n_rows" class="fn">with_n_rows</a>(self, num_rows: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

Stop reading when `n` rows are read.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.with_columns" class="fn">with_columns</a>(self, columns: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/string/struct.String.html" class="struct" title="struct alloc::string::String">String</a>\>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

Columns to select/ project

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.with_row_index" class="fn">with_row_index</a>(self, row_index: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/options/struct.RowIndex.html" class="struct" title="struct polars_io::options::RowIndex">RowIndex</a>\>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

Add a row index column.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.with_projection" class="fn">with_projection</a>( self, projection: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

Set the reader’s column projection. This counts from 0, meaning that `vec![0, 4]` would select the 1st and 5th column.

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#impl-SerReader%3CR%3E-for-IpcStreamReader%3CR%3E" class="anchor">§</a>

### impl\<R\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html" class="trait" title="trait polars::prelude::SerReader">SerReader</a>\<R\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

where R: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Read.html" class="trait" title="trait std::io::Read">Read</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.new" class="fn">new</a>(reader: R) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

Create a new instance of the [`SerReader`](https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html "trait polars::prelude::SerReader")

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.set_rechunk" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#method.set_rechunk" class="fn">set_rechunk</a>(self, rechunk: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html" class="struct" title="struct polars::prelude::IpcStreamReader">IpcStreamReader</a>\<R\>

Make sure that all columns are contiguous in memory by aggregating the chunks into a single array.

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerReader.html#tymethod.finish" class="fn">finish</a>(self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Take the SerReader and return a parsed DataFrame.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamReader.html#blanket-implementations" class="anchor">§</a>
