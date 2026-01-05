# Struct IpcStreamWriter Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/ipc/ipc_stream.rs.html#224" class="src">Source</a>

``` rust
pub struct IpcStreamWriter<W> { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

Expand description

Write a DataFrame to Arrow’s Streaming IPC format

## <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#example" class="doc-anchor">§</a>Example

``` rust
use polars_core::prelude::*;
use polars_io::ipc::IpcStreamWriter;
use std::fs::File;
use polars_io::SerWriter;

fn example(df: &mut DataFrame) -> PolarsResult<()> {
    let mut file = File::create("file.ipc").expect("could not create file");

    let mut writer = IpcStreamWriter::new(&mut file);

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

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#impl-IpcStreamWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html" class="struct" title="struct polars::prelude::IpcStreamWriter">IpcStreamWriter</a>\<W\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#method.with_compression" class="fn">with_compression</a>( self, compression: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.IpcCompression.html" class="enum" title="enum polars::prelude::IpcCompression">IpcCompression</a>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html" class="struct" title="struct polars::prelude::IpcStreamWriter">IpcStreamWriter</a>\<W\>

Set the compression used. Defaults to None.

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#method.with_compat_level" class="fn">with_compat_level</a>(self, compat_level: <a href="https://docs.rs/polars/latest/polars/prelude/struct.CompatLevel.html" class="struct" title="struct polars::prelude::CompatLevel">CompatLevel</a>) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html" class="struct" title="struct polars::prelude::IpcStreamWriter">IpcStreamWriter</a>\<W\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#method.set_custom_schema_metadata" class="fn">set_custom_schema_metadata</a>( &mut self, custom_metadata: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://doc.rust-lang.org/nightly/alloc/collections/btree/map/struct.BTreeMap.html" class="struct" title="struct alloc::collections::btree::map::BTreeMap">BTreeMap</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>\>\>, )

Sets custom schema metadata. Must be called before `start` is called

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#impl-SerWriter%3CW%3E-for-IpcStreamWriter%3CW%3E" class="anchor">§</a>

### impl\<W\> <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html" class="trait" title="trait polars::prelude::SerWriter">SerWriter</a>\<W\> for <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html" class="struct" title="struct polars::prelude::IpcStreamWriter">IpcStreamWriter</a>\<W\>

where W: <a href="https://doc.rust-lang.org/nightly/std/io/trait.Write.html" class="trait" title="trait std::io::Write">Write</a>,

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#method.new" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.new" class="fn">new</a>(writer: W) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html" class="struct" title="struct polars::prelude::IpcStreamWriter">IpcStreamWriter</a>\<W\>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#method.finish" class="anchor">§</a>

#### fn <a href="https://docs.rs/polars/latest/polars/prelude/trait.SerWriter.html#tymethod.finish" class="fn">finish</a>(&mut self, df: &mut <a href="https://docs.rs/polars/latest/polars/prelude/struct.DataFrame.html" class="struct" title="struct polars::prelude::DataFrame">DataFrame</a>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.IpcStreamWriter.html#blanket-implementations" class="anchor">§</a>
