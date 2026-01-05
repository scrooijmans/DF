# Struct IpcDataGenerator Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/writer.rs.html#181" class="src">Source</a>

``` rust
pub struct IpcDataGenerator {}
```

Expand description

Handles low level details of encoding [`Array`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/trait.Array.html "trait datafusion::common::arrow::array::Array") and [`Schema`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html "struct datafusion::common::arrow::datatypes::Schema") into the [Arrow IPC Format](https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc).

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#example" class="doc-anchor">§</a>Example

``` rust

// Create a record batch
let batch = RecordBatch::try_from_iter(vec![
 ("col2", Arc::new(UInt64Array::from_iter([10, 23, 33])) as _)
]).unwrap();

// Error of dictionary ids are replaced.
let error_on_replacement = true;
let options = IpcWriteOptions::default();
let mut dictionary_tracker = DictionaryTracker::new(error_on_replacement);

// encode the batch into zero or more encoded dictionaries
// and the data for the actual array.
let data_gen = IpcDataGenerator::default();
let (encoded_dictionaries, encoded_message) = data_gen
  .encoded_batch(&batch, &mut dictionary_tracker, &options)
  .unwrap();
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#impl-IpcDataGenerator" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::IpcDataGenerator">IpcDataGenerator</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#method.schema_to_bytes_with_dictionary_tracker" class="fn">schema_to_bytes_with_dictionary_tracker</a>( &self, schema: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::datatypes::Schema">Schema</a>, dictionary_tracker: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.DictionaryTracker.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::DictionaryTracker">DictionaryTracker</a>, write_options: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcWriteOptions.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::IpcWriteOptions">IpcWriteOptions</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.EncodedData.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::EncodedData">EncodedData</a>

Converts a schema to an IPC message along with `dictionary_tracker` and returns it encoded inside [EncodedData](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.EncodedData.html "struct datafusion::common::arrow::ipc::writer::EncodedData") as a flatbuffer.

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#method.encoded_batch" class="fn">encoded_batch</a>( &self, batch: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::array::RecordBatch">RecordBatch</a>, dictionary_tracker: &mut <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.DictionaryTracker.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::DictionaryTracker">DictionaryTracker</a>, write_options: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcWriteOptions.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::IpcWriteOptions">IpcWriteOptions</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<(<a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.EncodedData.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::EncodedData">EncodedData</a>\>, <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.EncodedData.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::EncodedData">EncodedData</a>), <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/error/enum.ArrowError.html" class="enum" title="enum datafusion::common::arrow::error::ArrowError">ArrowError</a>\>

Encodes a batch to a number of [EncodedData](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.EncodedData.html "struct datafusion::common::arrow::ipc::writer::EncodedData") items (dictionary batches + the record batch). The [DictionaryTracker](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.DictionaryTracker.html "struct datafusion::common::arrow::ipc::writer::DictionaryTracker") keeps track of dictionaries with new `dict_id`s (so they are only sent once) Make sure the [DictionaryTracker](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.DictionaryTracker.html "struct datafusion::common::arrow::ipc::writer::DictionaryTracker") is initialized at the start of the stream.

## Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#impl-Debug-for-IpcDataGenerator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::IpcDataGenerator">IpcDataGenerator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#impl-Default-for-IpcDataGenerator" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html" class="trait" title="trait core::default::Default">Default</a> for <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::IpcDataGenerator">IpcDataGenerator</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#method.default" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default" class="fn">default</a>() -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html" class="struct" title="struct datafusion::common::arrow::ipc::writer::IpcDataGenerator">IpcDataGenerator</a>

Returns the “default value” for a type. [Read more](https://doc.rust-lang.org/nightly/core/default/trait.Default.html#tymethod.default)

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.IpcDataGenerator.html#blanket-implementations" class="anchor">§</a>
