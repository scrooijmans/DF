# Crate ipc Copy item path

<a href="https://docs.rs/arrow-ipc/56.0.0/x86_64-unknown-linux-gnu/src/arrow_ipc/lib.rs.html#18-80" class="src">Source</a>

Expand description

Support for the [Arrow IPC Format](https://arrow.apache.org/docs/format/Columnar.html#serialization-and-interprocess-communication-ipc)

The Arrow IPC format defines how to read and write [`RecordBatch`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RecordBatch.html "struct datafusion::common::arrow::ipc::RecordBatch")es to/from a file or stream of bytes. This format can be used to serialize and deserialize data to files and over the network.

There are two variants of the IPC format:

1.  [IPC Streaming Format](https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format): Supports streaming data sources, implemented by [StreamReader](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/reader/struct.StreamReader.html "struct datafusion::common::arrow::ipc::reader::StreamReader") and [StreamWriter](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.StreamWriter.html "struct datafusion::common::arrow::ipc::writer::StreamWriter")

2.  [IPC File Format](https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format): Supports random access, implemented by [FileReader](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/reader/struct.FileReader.html "struct datafusion::common::arrow::ipc::reader::FileReader") and [FileWriter](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/struct.FileWriter.html "struct datafusion::common::arrow::ipc::writer::FileWriter").

See the [`reader`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/reader/index.html "mod datafusion::common::arrow::ipc::reader") and [`writer`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/index.html "mod datafusion::common::arrow::ipc::writer") modules for more information.

## Modules<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/index.html#modules" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/convert/index.html" class="mod" title="mod datafusion::common::arrow::ipc::convert">convert</a>  
Utilities for converting between IPC types and native Arrow types

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/gen/index.html" class="mod" title="mod datafusion::common::arrow::ipc::gen">gen</a>  
Generated code

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/reader/index.html" class="mod" title="mod datafusion::common::arrow::ipc::reader">reader</a>  
Arrow IPC File and Stream Readers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/writer/index.html" class="mod" title="mod datafusion::common::arrow::ipc::writer">writer</a>  
Arrow IPC File and Stream Writers

## Structs<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Binary.html" class="struct" title="struct datafusion::common::arrow::ipc::Binary">Binary</a>

Opaque binary data

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BinaryArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::BinaryArgs">BinaryArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BinaryBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::BinaryBuilder">BinaryBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BinaryView.html" class="struct" title="struct datafusion::common::arrow::ipc::BinaryView">BinaryView</a>

Logically the same as Binary, but the internal representation uses a view struct that contains the string length and either the string’s entire data inline (for small strings) or an inlined prefix, an index of another buffer, and an offset pointing to a slice in that buffer (for non-small strings).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BinaryViewArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::BinaryViewArgs">BinaryViewArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BinaryViewBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::BinaryViewBuilder">BinaryViewBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Block.html" class="struct" title="struct datafusion::common::arrow::ipc::Block">Block</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BodyCompression.html" class="struct" title="struct datafusion::common::arrow::ipc::BodyCompression">BodyCompression</a>

Optional compression for the memory buffers constituting IPC message bodies. Intended for use with RecordBatch but could be used for other message types

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BodyCompressionArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::BodyCompressionArgs">BodyCompressionArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BodyCompressionBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::BodyCompressionBuilder">BodyCompressionBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BodyCompressionMethod.html" class="struct" title="struct datafusion::common::arrow::ipc::BodyCompressionMethod">BodyCompressionMethod</a>

Provided for forward compatibility in case we need to support different strategies for compressing the IPC message body (like whole-body compression rather than buffer-level) in the future

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Bool.html" class="struct" title="struct datafusion::common::arrow::ipc::Bool">Bool</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BoolArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::BoolArgs">BoolArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.BoolBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::BoolBuilder">BoolBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Buffer.html" class="struct" title="struct datafusion::common::arrow::ipc::Buffer">Buffer</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.CompressionType.html" class="struct" title="struct datafusion::common::arrow::ipc::CompressionType">CompressionType</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Date.html" class="struct" title="struct datafusion::common::arrow::ipc::Date">Date</a>

Date is either a 32-bit or 64-bit signed integer type representing an elapsed time since UNIX epoch (1970-01-01), stored in either of two units:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DateArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::DateArgs">DateArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DateBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::DateBuilder">DateBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DateUnit.html" class="struct" title="struct datafusion::common::arrow::ipc::DateUnit">DateUnit</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Decimal.html" class="struct" title="struct datafusion::common::arrow::ipc::Decimal">Decimal</a>

Exact decimal value represented as an integer value in two’s complement. Currently only 128-bit (16-byte) and 256-bit (32-byte) integers are used. The representation uses the endianness indicated in the Schema.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DecimalArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::DecimalArgs">DecimalArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DecimalBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::DecimalBuilder">DecimalBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryBatch.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryBatch">DictionaryBatch</a>

For sending dictionary encoding information. Any Field can be dictionary-encoded, but in this case none of its children may be dictionary-encoded. There is one vector / column per dictionary, but that vector / column may be spread across multiple dictionary batches by using the isDelta flag

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryBatchArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryBatchArgs">DictionaryBatchArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryBatchBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryBatchBuilder">DictionaryBatchBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncoding.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryEncoding">DictionaryEncoding</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryEncodingArgs">DictionaryEncodingArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryEncodingBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryEncodingBuilder">DictionaryEncodingBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DictionaryKind.html" class="struct" title="struct datafusion::common::arrow::ipc::DictionaryKind">DictionaryKind</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Duration.html" class="struct" title="struct datafusion::common::arrow::ipc::Duration">Duration</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DurationArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::DurationArgs">DurationArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.DurationBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::DurationBuilder">DurationBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Endianness.html" class="struct" title="struct datafusion::common::arrow::ipc::Endianness">Endianness</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Feature.html" class="struct" title="struct datafusion::common::arrow::ipc::Feature">Feature</a>

Represents Arrow Features that might not have full support within implementations. This is intended to be used in two scenarios:

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Field.html" class="struct" title="struct datafusion::common::arrow::ipc::Field">Field</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FieldArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::FieldArgs">FieldArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FieldBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::FieldBuilder">FieldBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FieldNode.html" class="struct" title="struct datafusion::common::arrow::ipc::FieldNode">FieldNode</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeBinary.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeBinary">FixedSizeBinary</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeBinaryArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeBinaryArgs">FixedSizeBinaryArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeBinaryBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeBinaryBuilder">FixedSizeBinaryBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeList.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeList">FixedSizeList</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeListArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeListArgs">FixedSizeListArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FixedSizeListBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::FixedSizeListBuilder">FixedSizeListBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FloatingPoint.html" class="struct" title="struct datafusion::common::arrow::ipc::FloatingPoint">FloatingPoint</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FloatingPointArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::FloatingPointArgs">FloatingPointArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FloatingPointBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::FloatingPointBuilder">FloatingPointBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Footer.html" class="struct" title="struct datafusion::common::arrow::ipc::Footer">Footer</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FooterArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::FooterArgs">FooterArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.FooterBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::FooterBuilder">FooterBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Int.html" class="struct" title="struct datafusion::common::arrow::ipc::Int">Int</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.IntArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::IntArgs">IntArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.IntBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::IntBuilder">IntBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Interval.html" class="struct" title="struct datafusion::common::arrow::ipc::Interval">Interval</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.IntervalArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::IntervalArgs">IntervalArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.IntervalBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::IntervalBuilder">IntervalBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.IntervalUnit.html" class="struct" title="struct datafusion::common::arrow::ipc::IntervalUnit">IntervalUnit</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.KeyValue.html" class="struct" title="struct datafusion::common::arrow::ipc::KeyValue">KeyValue</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.KeyValueArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::KeyValueArgs">KeyValueArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.KeyValueBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::KeyValueBuilder">KeyValueBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeBinary.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeBinary">LargeBinary</a>

Same as Binary, but with 64-bit offsets, allowing to represent extremely large data values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeBinaryArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeBinaryArgs">LargeBinaryArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeBinaryBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeBinaryBuilder">LargeBinaryBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeList.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeList">LargeList</a>

Same as List, but with 64-bit offsets, allowing to represent extremely large data values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeListArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeListArgs">LargeListArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeListBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeListBuilder">LargeListBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeListView.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeListView">LargeListView</a>

Same as ListView, but with 64-bit offsets and sizes, allowing to represent extremely large data values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeListViewArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeListViewArgs">LargeListViewArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeListViewBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeListViewBuilder">LargeListViewBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeUtf8.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeUtf8">LargeUtf8</a>

Same as Utf8, but with 64-bit offsets, allowing to represent extremely large data values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeUtf8Args.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeUtf8Args">LargeUtf8Args</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.LargeUtf8Builder.html" class="struct" title="struct datafusion::common::arrow::ipc::LargeUtf8Builder">LargeUtf8Builder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.List.html" class="struct" title="struct datafusion::common::arrow::ipc::List">List</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.ListArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::ListArgs">ListArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.ListBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::ListBuilder">ListBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.ListView.html" class="struct" title="struct datafusion::common::arrow::ipc::ListView">ListView</a>

Represents the same logical types that List can, but contains offsets and sizes allowing for writes in any order and sharing of child values among list values.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.ListViewArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::ListViewArgs">ListViewArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.ListViewBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::ListViewBuilder">ListViewBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Map.html" class="struct" title="struct datafusion::common::arrow::ipc::Map">Map</a>

A Map is a logical nested type that is represented as

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MapArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::MapArgs">MapArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MapBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::MapBuilder">MapBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Message.html" class="struct" title="struct datafusion::common::arrow::ipc::Message">Message</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MessageArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::MessageArgs">MessageArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MessageBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::MessageBuilder">MessageBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MessageHeader.html" class="struct" title="struct datafusion::common::arrow::ipc::MessageHeader">MessageHeader</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MessageHeaderUnionTableOffset.html" class="struct" title="struct datafusion::common::arrow::ipc::MessageHeaderUnionTableOffset">MessageHeaderUnionTableOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.MetadataVersion.html" class="struct" title="struct datafusion::common::arrow::ipc::MetadataVersion">MetadataVersion</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Null.html" class="struct" title="struct datafusion::common::arrow::ipc::Null">Null</a>

These are stored in the flatbuffer in the Type union below

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.NullArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::NullArgs">NullArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.NullBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::NullBuilder">NullBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Precision.html" class="struct" title="struct datafusion::common::arrow::ipc::Precision">Precision</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RecordBatch.html" class="struct" title="struct datafusion::common::arrow::ipc::RecordBatch">RecordBatch</a>

A data header describing the shared memory layout of a “record” or “row” batch. Some systems call this a “row batch” internally and others a “record batch”.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RecordBatchArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::RecordBatchArgs">RecordBatchArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RecordBatchBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::RecordBatchBuilder">RecordBatchBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RunEndEncoded.html" class="struct" title="struct datafusion::common::arrow::ipc::RunEndEncoded">RunEndEncoded</a>

Contains two child arrays, run_ends and values. The run_ends child array must be a 16/32/64-bit integer array which encodes the indices at which the run with the value in each corresponding index in the values child array ends. Like list/struct types, the value array can be of any type.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RunEndEncodedArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::RunEndEncodedArgs">RunEndEncodedArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.RunEndEncodedBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::RunEndEncodedBuilder">RunEndEncodedBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Schema.html" class="struct" title="struct datafusion::common::arrow::ipc::Schema">Schema</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SchemaArgs">SchemaArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SchemaBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SchemaBuilder">SchemaBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixCompressedAxis.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixCompressedAxis">SparseMatrixCompressedAxis</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSX.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSX">SparseMatrixIndexCSX</a>

Compressed Sparse format, that is matrix-specific.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSXArgs">SparseMatrixIndexCSXArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseMatrixIndexCSXBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseMatrixIndexCSXBuilder">SparseMatrixIndexCSXBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensor.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensor">SparseTensor</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorArgs">SparseTensorArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorBuilder">SparseTensorBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndex.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndex">SparseTensorIndex</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOO.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOO">SparseTensorIndexCOO</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOOArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOOArgs">SparseTensorIndexCOOArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCOOBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCOOBuilder">SparseTensorIndexCOOBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSF.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSF">SparseTensorIndexCSF</a>

Compressed Sparse Fiber (CSF) sparse tensor index.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSFArgs">SparseTensorIndexCSFArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexCSFBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexCSFBuilder">SparseTensorIndexCSFBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.SparseTensorIndexUnionTableOffset.html" class="struct" title="struct datafusion::common::arrow::ipc::SparseTensorIndexUnionTableOffset">SparseTensorIndexUnionTableOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Struct_.html" class="struct" title="struct datafusion::common::arrow::ipc::Struct_">Struct_</a>

A Struct\_ in the flatbuffer metadata is the same as an Arrow Struct (according to the physical memory layout). We used Struct\_ here as Struct is a reserved word in Flatbuffers

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Struct_Args.html" class="struct" title="struct datafusion::common::arrow::ipc::Struct_Args">Struct_Args</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Struct_Builder.html" class="struct" title="struct datafusion::common::arrow::ipc::Struct_Builder">Struct_Builder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Tensor.html" class="struct" title="struct datafusion::common::arrow::ipc::Tensor">Tensor</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorArgs">TensorArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorBuilder">TensorBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorDim.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorDim">TensorDim</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorDimArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorDimArgs">TensorDimArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TensorDimBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::TensorDimBuilder">TensorDimBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Time.html" class="struct" title="struct datafusion::common::arrow::ipc::Time">Time</a>

Time is either a 32-bit or 64-bit signed integer type representing an elapsed time since midnight, stored in either of four units: seconds, milliseconds, microseconds or nanoseconds.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimeArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::TimeArgs">TimeArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimeBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::TimeBuilder">TimeBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimeUnit.html" class="struct" title="struct datafusion::common::arrow::ipc::TimeUnit">TimeUnit</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Timestamp.html" class="struct" title="struct datafusion::common::arrow::ipc::Timestamp">Timestamp</a>

Timestamp is a 64-bit signed integer representing an elapsed time since a fixed epoch, stored in either of four units: seconds, milliseconds, microseconds or nanoseconds, and is optionally annotated with a timezone.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::TimestampArgs">TimestampArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TimestampBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::TimestampBuilder">TimestampBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Type.html" class="struct" title="struct datafusion::common::arrow::ipc::Type">Type</a>

------------------------------------------------------------------------

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.TypeUnionTableOffset.html" class="struct" title="struct datafusion::common::arrow::ipc::TypeUnionTableOffset">TypeUnionTableOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Union.html" class="struct" title="struct datafusion::common::arrow::ipc::Union">Union</a>

A union is a complex type with children in Field By default ids in the type vector refer to the offsets in the children optionally typeIds provides an indirection between the child offset and the type id for each child `typeIds[offset]` is the id used in the type vector

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.UnionArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::UnionArgs">UnionArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.UnionBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::UnionBuilder">UnionBuilder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.UnionMode.html" class="struct" title="struct datafusion::common::arrow::ipc::UnionMode">UnionMode</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8">Utf8</a>

Unicode with UTF-8 encoding

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8Args.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8Args">Utf8Args</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8Builder.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8Builder">Utf8Builder</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8View.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8View">Utf8View</a>

Logically the same as Utf8, but the internal representation uses a view struct that contains the string length and either the string’s entire data inline (for small strings) or an inlined prefix, an index of another buffer, and an offset pointing to a slice in that buffer (for non-small strings).

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8ViewArgs.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8ViewArgs">Utf8ViewArgs</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/struct.Utf8ViewBuilder.html" class="struct" title="struct datafusion::common::arrow::ipc::Utf8ViewBuilder">Utf8ViewBuilder</a>

## Enums<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/index.html#enums" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.BinaryOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::BinaryOffset">BinaryOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.BinaryViewOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::BinaryViewOffset">BinaryViewOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.BodyCompressionOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::BodyCompressionOffset">BodyCompressionOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.BoolOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::BoolOffset">BoolOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.DateOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::DateOffset">DateOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.DecimalOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::DecimalOffset">DecimalOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.DictionaryBatchOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::DictionaryBatchOffset">DictionaryBatchOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.DictionaryEncodingOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::DictionaryEncodingOffset">DictionaryEncodingOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.DurationOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::DurationOffset">DurationOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.FieldOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::FieldOffset">FieldOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.FixedSizeBinaryOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::FixedSizeBinaryOffset">FixedSizeBinaryOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.FixedSizeListOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::FixedSizeListOffset">FixedSizeListOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.FloatingPointOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::FloatingPointOffset">FloatingPointOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.FooterOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::FooterOffset">FooterOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.IntOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::IntOffset">IntOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.IntervalOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::IntervalOffset">IntervalOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.KeyValueOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::KeyValueOffset">KeyValueOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.LargeBinaryOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::LargeBinaryOffset">LargeBinaryOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.LargeListOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::LargeListOffset">LargeListOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.LargeListViewOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::LargeListViewOffset">LargeListViewOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.LargeUtf8Offset.html" class="enum" title="enum datafusion::common::arrow::ipc::LargeUtf8Offset">LargeUtf8Offset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.ListOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::ListOffset">ListOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.ListViewOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::ListViewOffset">ListViewOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.MapOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::MapOffset">MapOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.MessageOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::MessageOffset">MessageOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.NullOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::NullOffset">NullOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.RecordBatchOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::RecordBatchOffset">RecordBatchOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.RunEndEncodedOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::RunEndEncodedOffset">RunEndEncodedOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.SchemaOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::SchemaOffset">SchemaOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.SparseMatrixIndexCSXOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::SparseMatrixIndexCSXOffset">SparseMatrixIndexCSXOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.SparseTensorIndexCOOOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::SparseTensorIndexCOOOffset">SparseTensorIndexCOOOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.SparseTensorIndexCSFOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::SparseTensorIndexCSFOffset">SparseTensorIndexCSFOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.SparseTensorOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::SparseTensorOffset">SparseTensorOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.Struct_Offset.html" class="enum" title="enum datafusion::common::arrow::ipc::Struct_Offset">Struct_Offset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.TensorDimOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::TensorDimOffset">TensorDimOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.TensorOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::TensorOffset">TensorOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.TimeOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::TimeOffset">TimeOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.TimestampOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::TimestampOffset">TimestampOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.UnionOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::UnionOffset">UnionOffset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.Utf8Offset.html" class="enum" title="enum datafusion::common::arrow::ipc::Utf8Offset">Utf8Offset</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/enum.Utf8ViewOffset.html" class="enum" title="enum datafusion::common::arrow::ipc::Utf8ViewOffset">Utf8ViewOffset</a>

## Constants<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/index.html#constants" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_BODY_COMPRESSION_METHOD.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_BODY_COMPRESSION_METHOD">ENUM_MAX_BODY_COMPRESSION_METHOD</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_COMPRESSION_TYPE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_COMPRESSION_TYPE">ENUM_MAX_COMPRESSION_TYPE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_DATE_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_DATE_UNIT">ENUM_MAX_DATE_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_DICTIONARY_KIND.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_DICTIONARY_KIND">ENUM_MAX_DICTIONARY_KIND</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_ENDIANNESS.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_ENDIANNESS">ENUM_MAX_ENDIANNESS</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_FEATURE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_FEATURE">ENUM_MAX_FEATURE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_INTERVAL_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_INTERVAL_UNIT">ENUM_MAX_INTERVAL_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_MESSAGE_HEADER.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_MESSAGE_HEADER">ENUM_MAX_MESSAGE_HEADER</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_METADATA_VERSION.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_METADATA_VERSION">ENUM_MAX_METADATA_VERSION</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_PRECISION.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_PRECISION">ENUM_MAX_PRECISION</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS">ENUM_MAX_SPARSE_MATRIX_COMPRESSED_AXIS</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_SPARSE_TENSOR_INDEX.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_SPARSE_TENSOR_INDEX">ENUM_MAX_SPARSE_TENSOR_INDEX</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_TIME_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_TIME_UNIT">ENUM_MAX_TIME_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_TYPE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_TYPE">ENUM_MAX_TYPE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MAX_UNION_MODE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MAX_UNION_MODE">ENUM_MAX_UNION_MODE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_BODY_COMPRESSION_METHOD.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_BODY_COMPRESSION_METHOD">ENUM_MIN_BODY_COMPRESSION_METHOD</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_COMPRESSION_TYPE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_COMPRESSION_TYPE">ENUM_MIN_COMPRESSION_TYPE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_DATE_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_DATE_UNIT">ENUM_MIN_DATE_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_DICTIONARY_KIND.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_DICTIONARY_KIND">ENUM_MIN_DICTIONARY_KIND</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_ENDIANNESS.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_ENDIANNESS">ENUM_MIN_ENDIANNESS</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_FEATURE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_FEATURE">ENUM_MIN_FEATURE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_INTERVAL_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_INTERVAL_UNIT">ENUM_MIN_INTERVAL_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_MESSAGE_HEADER.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_MESSAGE_HEADER">ENUM_MIN_MESSAGE_HEADER</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_METADATA_VERSION.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_METADATA_VERSION">ENUM_MIN_METADATA_VERSION</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_PRECISION.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_PRECISION">ENUM_MIN_PRECISION</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS">ENUM_MIN_SPARSE_MATRIX_COMPRESSED_AXIS</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_SPARSE_TENSOR_INDEX.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_SPARSE_TENSOR_INDEX">ENUM_MIN_SPARSE_TENSOR_INDEX</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_TIME_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_TIME_UNIT">ENUM_MIN_TIME_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_TYPE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_TYPE">ENUM_MIN_TYPE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_MIN_UNION_MODE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_MIN_UNION_MODE">ENUM_MIN_UNION_MODE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_BODY_COMPRESSION_METHOD.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_BODY_COMPRESSION_METHOD">ENUM_VALUES_BODY_COMPRESSION_METHOD</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_COMPRESSION_TYPE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_COMPRESSION_TYPE">ENUM_VALUES_COMPRESSION_TYPE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_DATE_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_DATE_UNIT">ENUM_VALUES_DATE_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_DICTIONARY_KIND.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_DICTIONARY_KIND">ENUM_VALUES_DICTIONARY_KIND</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_ENDIANNESS.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_ENDIANNESS">ENUM_VALUES_ENDIANNESS</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_FEATURE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_FEATURE">ENUM_VALUES_FEATURE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_INTERVAL_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_INTERVAL_UNIT">ENUM_VALUES_INTERVAL_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_MESSAGE_HEADER.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_MESSAGE_HEADER">ENUM_VALUES_MESSAGE_HEADER</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_METADATA_VERSION.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_METADATA_VERSION">ENUM_VALUES_METADATA_VERSION</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_PRECISION.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_PRECISION">ENUM_VALUES_PRECISION</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS">ENUM_VALUES_SPARSE_MATRIX_COMPRESSED_AXIS</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_SPARSE_TENSOR_INDEX.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_SPARSE_TENSOR_INDEX">ENUM_VALUES_SPARSE_TENSOR_INDEX</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_TIME_UNIT.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_TIME_UNIT">ENUM_VALUES_TIME_UNIT</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_TYPE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_TYPE">ENUM_VALUES_TYPE</a>Deprecated

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/constant.ENUM_VALUES_UNION_MODE.html" class="constant" title="constant datafusion::common::arrow::ipc::ENUM_VALUES_UNION_MODE">ENUM_VALUES_UNION_MODE</a>Deprecated

## Functions<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/index.html#functions" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_footer_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_footer_buffer">finish_footer_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_message_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_message_buffer">finish_message_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_schema_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_schema_buffer">finish_schema_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_size_prefixed_footer_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_size_prefixed_footer_buffer">finish_size_prefixed_footer_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_size_prefixed_message_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_size_prefixed_message_buffer">finish_size_prefixed_message_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_size_prefixed_schema_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_size_prefixed_schema_buffer">finish_size_prefixed_schema_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_size_prefixed_sparse_tensor_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_size_prefixed_sparse_tensor_buffer">finish_size_prefixed_sparse_tensor_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_size_prefixed_tensor_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_size_prefixed_tensor_buffer">finish_size_prefixed_tensor_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_sparse_tensor_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_sparse_tensor_buffer">finish_sparse_tensor_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.finish_tensor_buffer.html" class="fn" title="fn datafusion::common::arrow::ipc::finish_tensor_buffer">finish_tensor_buffer</a>  
<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_footer.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_footer">root_as_footer</a>  
Verifies that a buffer of bytes contains a `Footer` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_footer_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_footer_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_footer_unchecked">root_as_footer_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a Footer and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_footer_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_footer_with_opts">root_as_footer_with_opts</a>  
Verifies, with the given options, that a buffer of bytes contains a `Footer` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_footer_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_message.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_message">root_as_message</a>  
Verifies that a buffer of bytes contains a `Message` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_message_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_message_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_message_unchecked">root_as_message_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a Message and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_message_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_message_with_opts">root_as_message_with_opts</a>  
Verifies, with the given options, that a buffer of bytes contains a `Message` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_message_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_schema.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_schema">root_as_schema</a>  
Verifies that a buffer of bytes contains a `Schema` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_schema_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_schema_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_schema_unchecked">root_as_schema_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a Schema and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_schema_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_schema_with_opts">root_as_schema_with_opts</a>  
Verifies, with the given options, that a buffer of bytes contains a `Schema` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_schema_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_sparse_tensor.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_sparse_tensor">root_as_sparse_tensor</a>  
Verifies that a buffer of bytes contains a `SparseTensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_sparse_tensor_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_sparse_tensor_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_sparse_tensor_unchecked">root_as_sparse_tensor_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a SparseTensor and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_sparse_tensor_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_sparse_tensor_with_opts">root_as_sparse_tensor_with_opts</a>  
Verifies, with the given options, that a buffer of bytes contains a `SparseTensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_sparse_tensor_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_tensor.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_tensor">root_as_tensor</a>  
Verifies that a buffer of bytes contains a `Tensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_tensor_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_tensor_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_tensor_unchecked">root_as_tensor_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a Tensor and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.root_as_tensor_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::root_as_tensor_with_opts">root_as_tensor_with_opts</a>  
Verifies, with the given options, that a buffer of bytes contains a `Tensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_tensor_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_footer.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_footer">size_prefixed_root_as_footer</a>  
Verifies that a buffer of bytes contains a size prefixed `Footer` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `size_prefixed_root_as_footer_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_footer_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_footer_unchecked">size_prefixed_root_as_footer_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a size prefixed Footer and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_footer_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_footer_with_opts">size_prefixed_root_as_footer_with_opts</a>  
Verifies, with the given verifier options, that a buffer of bytes contains a size prefixed `Footer` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_footer_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_message.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_message">size_prefixed_root_as_message</a>  
Verifies that a buffer of bytes contains a size prefixed `Message` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `size_prefixed_root_as_message_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_message_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_message_unchecked">size_prefixed_root_as_message_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a size prefixed Message and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_message_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_message_with_opts">size_prefixed_root_as_message_with_opts</a>  
Verifies, with the given verifier options, that a buffer of bytes contains a size prefixed `Message` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_message_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_schema.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_schema">size_prefixed_root_as_schema</a>  
Verifies that a buffer of bytes contains a size prefixed `Schema` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `size_prefixed_root_as_schema_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_schema_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_schema_unchecked">size_prefixed_root_as_schema_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a size prefixed Schema and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_schema_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_schema_with_opts">size_prefixed_root_as_schema_with_opts</a>  
Verifies, with the given verifier options, that a buffer of bytes contains a size prefixed `Schema` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_schema_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_sparse_tensor.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_sparse_tensor">size_prefixed_root_as_sparse_tensor</a>  
Verifies that a buffer of bytes contains a size prefixed `SparseTensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `size_prefixed_root_as_sparse_tensor_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_sparse_tensor_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_sparse_tensor_unchecked">size_prefixed_root_as_sparse_tensor_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a size prefixed SparseTensor and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_sparse_tensor_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_sparse_tensor_with_opts">size_prefixed_root_as_sparse_tensor_with_opts</a>  
Verifies, with the given verifier options, that a buffer of bytes contains a size prefixed `SparseTensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_sparse_tensor_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_tensor.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_tensor">size_prefixed_root_as_tensor</a>  
Verifies that a buffer of bytes contains a size prefixed `Tensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `size_prefixed_root_as_tensor_unchecked`.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_tensor_unchecked.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_tensor_unchecked">size_prefixed_root_as_tensor_unchecked</a><sup>⚠</sup>  
Assumes, without verification, that a buffer of bytes contains a size prefixed Tensor and returns it.

<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/ipc/fn.size_prefixed_root_as_tensor_with_opts.html" class="fn" title="fn datafusion::common::arrow::ipc::size_prefixed_root_as_tensor_with_opts">size_prefixed_root_as_tensor_with_opts</a>  
Verifies, with the given verifier options, that a buffer of bytes contains a size prefixed `Tensor` and returns it. Note that verification is still experimental and may not catch every error, or be maximally performant. For the previous, unchecked, behavior use `root_as_tensor_unchecked`.
