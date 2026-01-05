# Struct ParquetObjectStore Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/parquet/read/async_impl.rs.html#13" class="src">Source</a>

``` rust
pub struct ParquetObjectStore { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#impl-ParquetObjectStore" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html" class="struct" title="struct polars::prelude::ParquetObjectStore">ParquetObjectStore</a>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#method.from_uri" class="fn">from_uri</a>( uri: &<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>, options: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/cloud/struct.CloudOptions.html" class="struct" title="struct polars::prelude::cloud::CloudOptions">CloudOptions</a>\>, metadata: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>\>\>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html" class="struct" title="struct polars::prelude::ParquetObjectStore">ParquetObjectStore</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#method.num_rows" class="fn">num_rows</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Number of rows in the parquet file.

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#method.get_metadata" class="fn">get_metadata</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<&<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.FileMetadata.html" class="struct" title="struct polars::prelude::FileMetadata">FileMetadata</a>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Fetch and memoize the metadata of the parquet file.

#### pub async fn <a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#method.schema" class="fn">schema</a>(&mut self) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.ArrowField.html" class="struct" title="struct polars::prelude::ArrowField">Field</a>\>\>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/struct.ParquetObjectStore.html#blanket-implementations" class="anchor">§</a>
