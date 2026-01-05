# Crate json Copy item path

<a href="https://docs.rs/arrow-json/56.0.0/x86_64-unknown-linux-gnu/src/arrow_json/lib.rs.html#18-264" class="src">Source</a>

Expand description

Transfer data between the Arrow memory format and JSON line-delimited records.

See the module level documentation for the [`reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/index.html "mod datafusion::common::arrow::json::reader") and [`writer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/index.html "mod datafusion::common::arrow::json::writer") for usage examples.

## <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html#binary-data" class="doc-anchor">§</a>Binary Data

As per [RFC7159](https://datatracker.ietf.org/doc/html/rfc7159#section-8.1) JSON cannot encode arbitrary binary data. A common approach to workaround this is to use a [binary-to-text encoding](https://en.wikipedia.org/wiki/Binary-to-text_encoding) scheme, such as base64, to encode the input data and then decode it on output.

``` rust
// The data we want to write
let input = BinaryArray::from(vec![b"\xDE\x00\xFF".as_ref()]);

// Base64 encode it to a string
let encoded: StringArray = b64_encode(&BASE64_STANDARD, &input);

// Write the StringArray to JSON
let batch = RecordBatch::try_from_iter([("col", Arc::new(encoded) as _)]).unwrap();
let mut buf = Vec::with_capacity(1024);
let mut writer = LineDelimitedWriter::new(&mut buf);
writer.write(&batch).unwrap();
writer.finish().unwrap();

// Read the JSON data
let cursor = Cursor::new(buf);
let mut reader = ReaderBuilder::new(batch.schema()).build(cursor).unwrap();
let batch = reader.next().unwrap().unwrap();

// Reverse the base64 encoding
let col: BinaryArray = batch.column(0).as_string::<i32>().clone().into();
let output = b64_decode(&BASE64_STANDARD, &col).unwrap();

assert_eq!(input, output);
```

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/index.html" class="mod" title="mod datafusion::common::arrow::json::reader">reader</a>  
JSON reader

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/writer/index.html" class="mod" title="mod datafusion::common::arrow::json::writer">writer</a>  
JSON Writer

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.EncoderOptions.html" class="struct" title="struct datafusion::common::arrow::json::EncoderOptions">EncoderOptions</a>  
Configuration options for the JSON encoder.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Reader.html" class="struct" title="struct datafusion::common::arrow::json::Reader">Reader</a>  
Reads JSON data with a known schema directly into arrow [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.ReaderBuilder.html" class="struct" title="struct datafusion::common::arrow::json::ReaderBuilder">ReaderBuilder</a>  
A builder for [`Reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Reader.html "struct datafusion::common::arrow::json::Reader") and [`Decoder`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/reader/struct.Decoder.html "struct datafusion::common::arrow::json::reader::Decoder")

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.Writer.html" class="struct" title="struct datafusion::common::arrow::json::Writer">Writer</a>  
A JSON writer which serializes [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to a stream of `u8` encoded JSON objects.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/struct.WriterBuilder.html" class="struct" title="struct datafusion::common::arrow::json::WriterBuilder">WriterBuilder</a>  
JSON writer builder.

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/enum.StructMode.html" class="enum" title="enum datafusion::common::arrow::json::StructMode">StructMode</a>  
Specifies what is considered valid JSON when reading or writing RecordBatches or StructArrays.

## Traits<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html#traits" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/trait.Encoder.html" class="trait" title="trait datafusion::common::arrow::json::Encoder">Encoder</a>  
A trait to format array values as JSON values

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/trait.EncoderFactory.html" class="trait" title="trait datafusion::common::arrow::json::EncoderFactory">EncoderFactory</a>  
A trait to create custom encoders for specific data types.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/trait.JsonSerializable.html" class="trait" title="trait datafusion::common::arrow::json::JsonSerializable">JsonSerializable</a>  
Trait declaring any type that is serializable to JSON. This includes all primitive types (bool, i32, etc.).

## Type Aliases<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/index.html#types" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/type.ArrayWriter.html" class="type" title="type datafusion::common::arrow::json::ArrayWriter">ArrayWriter</a>  
A JSON writer which serializes [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to JSON arrays.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/json/type.LineDelimitedWriter.html" class="type" title="type datafusion::common::arrow::json::LineDelimitedWriter">LineDelimitedWriter</a>  
A JSON writer which serializes [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.RecordBatch.html "struct datafusion::common::arrow::array::RecordBatch")es to newline delimited JSON objects.
