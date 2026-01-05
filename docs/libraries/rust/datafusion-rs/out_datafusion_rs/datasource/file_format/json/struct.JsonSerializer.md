# Struct JsonSerializer Copy item path

<a href="https://docs.rs/datafusion-datasource-json/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_datasource_json/file_format.rs.html#296" class="src">Source</a>

``` rust
pub struct JsonSerializer {}
```

Expand description

Define a struct for serializing Json records to a stream

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#impl-JsonSerializer" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSerializer">JsonSerializer</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#method.new" class="fn">new</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSerializer">JsonSerializer</a>

Constructor for the JsonSerializer object

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#impl-BatchSerializer-for-JsonSerializer" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/trait.BatchSerializer.html" class="trait" title="trait datafusion::datasource::file_format::write::BatchSerializer">BatchSerializer</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSerializer">JsonSerializer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#method.serialize" class="anchor">§</a>

#### fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/write/trait.BatchSerializer.html#tymethod.serialize" class="fn">serialize</a>( &self, batch: <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, \_initial: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/bytes/1.10.1/x86_64-unknown-linux-gnu/bytes/bytes/struct.Bytes.html" class="struct" title="struct bytes::bytes::Bytes">Bytes</a>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/error/enum.DataFusionError.html" class="enum" title="enum datafusion::error::DataFusionError">DataFusionError</a>\>

Asynchronously serializes a `RecordBatch` and returns the serialized bytes. Parameter `initial` signals whether the given batch is the first batch. This distinction is important for certain serializers (like CSV).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#impl-Default-for-JsonSerializer" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSerializer">JsonSerializer</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html" class="struct" title="struct datafusion::datasource::file_format::json::JsonSerializer">JsonSerializer</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/datasource/file_format/json/struct.JsonSerializer.html#blanket-implementations" class="anchor">§</a>
