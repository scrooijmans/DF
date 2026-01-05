# Struct SchemaInferenceResult Copy item path

<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/src/polars_io/csv/read/schema_inference.rs.html#18" class="src">Source</a>

``` rust
pub struct SchemaInferenceResult { /* private fields */ }
```

Available on **crate feature `polars-io`** only.

## Implementations<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#impl-SchemaInferenceResult" class="anchor">§</a>

### impl <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.try_from_reader_bytes_and_options" class="fn">try_from_reader_bytes_and_options</a>( reader_bytes: &<a href="https://docs.rs/polars-io/0.51.0/x86_64-unknown-linux-gnu/polars_io/mmap/enum.ReaderBytes.html" class="enum" title="enum polars_io::mmap::ReaderBytes">ReaderBytes</a>\<'\_\>, options: &<a href="https://docs.rs/polars/latest/polars/prelude/struct.CsvReadOptions.html" class="struct" title="struct polars::prelude::CsvReadOptions">CsvReadOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.with_inferred_schema" class="fn">with_inferred_schema</a>( self, inferred_schema: <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>, ) -\> <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.get_inferred_schema" class="fn">get_inferred_schema</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Arc.html" class="struct" title="struct polars::prelude::Arc">Arc</a>\<<a href="https://docs.rs/polars-schema/0.51.0/x86_64-unknown-linux-gnu/polars_schema/schema/struct.Schema.html" class="struct" title="struct polars_schema::schema::Schema">Schema</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>\>\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.get_estimated_n_rows" class="fn">get_estimated_n_rows</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#impl-Clone-for-SchemaInferenceResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#impl-Debug-for-SchemaInferenceResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#impl-Default-for-SchemaInferenceResult" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html" class="struct" title="struct polars::prelude::schema_inference::SchemaInferenceResult">SchemaInferenceResult</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/prelude/schema_inference/struct.SchemaInferenceResult.html#blanket-implementations" class="anchor">§</a>
