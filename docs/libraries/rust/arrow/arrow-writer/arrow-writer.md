# ArrowWriter in parquet::arrow::arrow_writer - Rust

## Struct ArrowWriter 

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#173-190)

```
pub struct ArrowWriter<W: Write> {
    writer: SerializedFileWriter<W>,
    in_progress: Option<ArrowRowGroupWriter>,
    arrow_schema: SchemaRef,
    row_group_writer_factory: ArrowRowGroupWriterFactory,
    max_row_group_size: usize,
}
```

Expand description

Encodes \[`RecordBatch`\] to parquet

Writes Arrow `RecordBatch`es to a Parquet writer. Multiple \[`RecordBatch`\] will be encoded to the same row group, up to `max_row_group_size` rows. Any remaining rows will be flushed on close, leading the final row group in the output file to potentially contain fewer than `max_row_group_size` rows

## [§](#example-writing-recordbatches)Example: Writing `RecordBatch`es

```
let col = Arc::new(Int64Array::from_iter_values([1, 2, 3])) as ArrayRef;
let to_write = RecordBatch::try_from_iter([("col", col)]).unwrap();

let mut buffer = Vec::new();
let mut writer = ArrowWriter::try_new(&mut buffer, to_write.schema(), None).unwrap();
writer.write(&to_write).unwrap();
writer.close().unwrap();

let mut reader = ParquetRecordBatchReader::try_new(Bytes::from(buffer), 1024).unwrap();
let read = reader.next().unwrap().unwrap();

assert_eq!(to_write, read);
```

## [§](#memory-usage-and-limiting)Memory Usage and Limiting

The nature of Parquet requires buffering of an entire row group before it can be flushed to the underlying writer. Data is mostly buffered in its encoded form, reducing memory usage. However, some data such as dictionary keys, large strings or very nested data may still result in non-trivial memory usage.

See Also:

- [`ArrowWriter::memory_size`](about:blank/struct.ArrowWriter.html#method.memory_size "method parquet::arrow::arrow_writer::ArrowWriter::memory_size"): the current memory usage of the writer.
- [`ArrowWriter::in_progress_size`](about:blank/struct.ArrowWriter.html#method.in_progress_size "method parquet::arrow::arrow_writer::ArrowWriter::in_progress_size"): Estimated size of the buffered row group,

Call [`Self::flush`](about:blank/struct.ArrowWriter.html#method.flush "method parquet::arrow::arrow_writer::ArrowWriter::flush") to trigger an early flush of a row group based on a memory threshold and/or global memory pressure. However, smaller row groups result in higher metadata overheads, and thus may worsen compression ratios and query performance.

```
writer.write(&batch).unwrap();
// Trigger an early flush if anticipated size exceeds 1_000_000
if writer.in_progress_size() > 1_000_000 {
    writer.flush().unwrap();
}
```

### [§](#type-support)Type Support

The writer supports writing all Arrow [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html)s that have a direct mapping to Parquet types including [`StructArray`](https://docs.rs/arrow/latest/arrow/array/struct.StructArray.html) and [`ListArray`](https://docs.rs/arrow/latest/arrow/array/type.ListArray.html).

The following are not supported:

- [`IntervalMonthDayNanoArray`](https://docs.rs/arrow/latest/arrow/array/type.IntervalMonthDayNanoArray.html): Parquet does not [support nanosecond intervals](https://github.com/apache/parquet-format/blob/master/LogicalTypes.md#interval).

### [§](#type-compatibility)Type Compatibility

The writer can write Arrow \[`RecordBatch`\]s that are logically equivalent. This means that for a given column, the writer can accept multiple Arrow [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html)s that contain the same value type.

For example, the following [`DataType`](https://docs.rs/arrow/latest/arrow/datatypes/enum.DataType.html)s are all logically equivalent and can be written to the same column:

- String, LargeString, StringView
- Binary, LargeBinary, BinaryView

The writer can will also accept both native and dictionary encoded arrays if the dictionaries contain compatible values.

```
let record_batch1 = RecordBatch::try_new(
   Arc::new(Schema::new(vec![Field::new("col", DataType::LargeUtf8, false)])),
   vec![Arc::new(LargeStringArray::from_iter_values(vec!["a", "b"]))]
 )
.unwrap();

let mut buffer = Vec::new();
let mut writer = ArrowWriter::try_new(&mut buffer, record_batch1.schema(), None).unwrap();
writer.write(&record_batch1).unwrap();

let record_batch2 = RecordBatch::try_new(
    Arc::new(Schema::new(vec![Field::new(
        "col",
        DataType::Dictionary(Box::new(DataType::UInt8), Box::new(DataType::Utf8)),
         false,
    )])),
    vec![Arc::new(DictionaryArray::new(
         UInt8Array::from_iter_values(vec![0, 1]),
         Arc::new(StringArray::from_iter_values(vec!["b", "c"])),
     ))],
 )
 .unwrap();
 writer.write(&record_batch2).unwrap();
 writer.close();
```

Underlying Parquet writer

The in-progress row group if any

A copy of the Arrow schema.

The schema is used to verify that each record batch written has the correct schema

The length of arrays to write to each row group

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#205-446)
[§](#impl-ArrowWriter%3CW%3E)

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#211-218)

Try to create a new Arrow writer

The writer will fail if:

- a `SerializedFileWriter` cannot be created from the ParquetWriter
- the Arrow schema contains unsupported datatypes such as Unions

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#225-264)

Try to create a new Arrow writer with [`ArrowWriterOptions`](struct.ArrowWriterOptions.html "struct parquet::arrow::arrow_writer::ArrowWriterOptions").

The writer will fail if:

- a `SerializedFileWriter` cannot be created from the ParquetWriter
- the Arrow schema contains unsupported datatypes such as Unions

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#267-269)

Returns metadata for any flushed row groups

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#275-280)

Estimated memory usage, in bytes, of this `ArrowWriter`

This estimate is formed bu summing the values of [`ArrowColumnWriter::memory_size`](about:blank/struct.ArrowColumnWriter.html#method.memory_size "method parquet::arrow::arrow_writer::ArrowColumnWriter::memory_size") all in progress columns.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#288-297)

Anticipated encoded size of the in progress row group.

This estimate the row group size after being completely encoded is, formed by summing the values of [`ArrowColumnWriter::get_estimated_total_bytes`](about:blank/struct.ArrowColumnWriter.html#method.get_estimated_total_bytes "method parquet::arrow::arrow_writer::ArrowColumnWriter::get_estimated_total_bytes") for all in progress columns.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#300-305)

Returns the number of rows buffered in the in progress row group

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#308-310)

Returns the number of bytes written by this instance

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#319-347)

Encodes the provided \[`RecordBatch`\]

If this would cause the current row group to exceed [`WriterProperties::max_row_group_size`](about:blank/file/properties/struct.WriterProperties.html#method.max_row_group_size "method parquet::file::properties::WriterProperties::max_row_group_size") rows, the contents of `batch` will be written to one or more row groups such that all but the final row group in the file contain [`WriterProperties::max_row_group_size`](about:blank/file/properties/struct.WriterProperties.html#method.max_row_group_size "method parquet::file::properties::WriterProperties::max_row_group_size") rows.

This will fail if the `batch`’s schema does not match the writer’s schema.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#353-355)

Writes the given buf bytes to the internal buffer.

It’s safe to use this method to write data to the underlying writer, because it will ensure that the buffering and byte‐counting layers are used.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#358-370)

Flushes all buffered rows into a new row group

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#375-377)

Additional [`KeyValue`](../../file/metadata/struct.KeyValue.html "struct parquet::file::metadata::KeyValue") metadata to be written in addition to those from [`WriterProperties`](../../file/properties/struct.WriterProperties.html "struct parquet::file::properties::WriterProperties")

This method provide a way to append kv_metadata after write RecordBatch

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#380-382)

Returns a reference to the underlying writer.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#392-394)

Returns a mutable reference to the underlying writer.

**Warning**: if you write directly to this writer, you will skip the `TrackedWrite` buffering and byte‐counting layers. That’ll cause the file footer’s recorded offsets and sizes to diverge from reality, resulting in an unreadable or corrupted Parquet file.

If you want to write safely to the underlying writer, use [`Self::write_all`](about:blank/struct.ArrowWriter.html#method.write_all "method parquet::arrow::arrow_writer::ArrowWriter::write_all").

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#397-400)

Flushes any outstanding data and returns the underlying writer.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#407-410)

Close and finalize the underlying Parquet writer

Unlike [`Self::close`](about:blank/struct.ArrowWriter.html#method.close "method parquet::arrow::arrow_writer::ArrowWriter::close") this does not consume self

Attempting to write after calling finish will result in an error

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#413-415)

Close and finalize the underlying Parquet writer

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#419-425)

👎Deprecated since 56.2.0: Use into_serialized_writer instead

Create a new row group writer and return its column writers.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#429-436)

👎Deprecated since 56.2.0: Use into_serialized_writer instead

Append the given column chunks to the file as a new row group.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#440-445)

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#192-203)
[§](#impl-Debug-for-ArrowWriter%3CW%3E)

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#448-457)
[§](#impl-RecordBatchWriter-for-ArrowWriter%3CW%3E)

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#449-451)
[§](#method.write-1)

Write a single batch to the writer.

[Source](about:blank/src/parquet/arrow/arrow_writer/mod.rs.html#453-456)
[§](#method.close-1)

Write footer or termination data, then mark the writer as done.
