# Struct IpcWriter Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ipc/write.rs.html#67" class="src">Source</a>

``` rust
pub struct IpcWriter<W> { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Write a DataFrame to Arrow’s IPC format

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_io::ipc::IpcWriter;
use std::fs::File;
use polars_io::SerWriter;

fn example(df: &mut DataFrame) -> PolarsResult<()> {
    let mut file = File::create("file.ipc").expect("could not create file");

    let mut writer = IpcWriter::new(&mut file);

    let custom_metadata = [
        ("first_name".into(), "John".into()),
        ("last_name".into(), "Doe".into()),
    ]
    .into_iter()
    .collect();
    writer.set_custom_schema_metadata(Arc::new(custom_metadata));
    writer.finish(df)
}
```

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#impl-IpcWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html" class="struct" title="struct polars::prelude::IpcWriter">IpcWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#method.with_compression" class="fn">with_compression</a>( self, compression: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.IpcCompression.html" class="enum" title="enum polars::prelude::IpcCompression">IpcCompression</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html" class="struct" title="struct polars::prelude::IpcWriter">IpcWriter</a>\<W\>

Set the compression used. Defaults to None.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#method.with_compat_level" class="fn">with_compat_level</a>(self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html" class="struct" title="struct polars::prelude::IpcWriter">IpcWriter</a>\<W\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#method.with_parallel" class="fn">with_parallel</a>(self, parallel: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html" class="struct" title="struct polars::prelude::IpcWriter">IpcWriter</a>\<W\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#method.batched" class="fn">batched</a>( self, schema: &<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/ipc/write/struct.BatchedWriter.html" class="struct" title="struct polars_io::ipc::write::BatchedWriter">BatchedWriter</a>\<W\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#method.set_custom_schema_metadata" class="fn">set_custom_schema_metadata</a>( &mut self, custom_metadata: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, )

Sets custom schema metadata. Must be called before `start` is called

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#impl-SerWriter%3CW%3E-for-IpcWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html" class="struct" title="struct polars::prelude::IpcWriter">IpcWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.new" class="fn">new</a>(writer: W) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html" class="struct" title="struct polars::prelude::IpcWriter">IpcWriter</a>\<W\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.finish" class="fn">finish</a>(&mut self, df: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcWriter.html#blanket-implementations" class="anchor">§</a>
