# StreamWriter in arrow_ipc::writer - Rust

## Struct StreamWriter 

[Source](about:blank/src/arrow_ipc/writer.rs.html#1331-1344)

```
pub struct StreamWriter<W> {
    writer: W,
    write_options: IpcWriteOptions,
    finished: bool,
    dictionary_tracker: DictionaryTracker,
    data_gen: IpcDataGenerator,
    compression_context: CompressionContext,
}
```

Expand description

Arrow Stream Writer

Writes Arrow \[`RecordBatch`\]es to bytes using the [IPC Streaming Format](https://arrow.apache.org/docs/format/Columnar.html#ipc-streaming-format).

## [§](#see-also)See Also

- [`FileWriter`](struct.FileWriter.html "struct arrow_ipc::writer::FileWriter") for writing IPC Files

## [§](#example---basic-usage)Example - Basic usage

```
let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// create a new writer, the schema must be known in advance
let mut writer = StreamWriter::try_new(&mut stream, &batch.schema()).unwrap();
// write each batch to the underlying stream
writer.write(&batch).unwrap();
// When all batches are written, call finish to flush all buffers
writer.finish().unwrap();
```

## [§](#example---efficient-delta-dictionaries)Example - Efficient delta dictionaries

```

let schema = Arc::new(Schema::new(vec![Field::new(
   "col1",
   DataType::Dictionary(Box::from(DataType::Int32), Box::from(DataType::Utf8)),
   true,
)]));

let mut builder = StringDictionaryBuilder::<arrow_array::types::Int32Type>::new();

// `finish_preserve_values` will keep the dictionary values along with their
// key assignments so that they can be re-used in the next batch.
builder.append("a").unwrap();
builder.append("b").unwrap();
let array1 = builder.finish_preserve_values();
let batch1 = RecordBatch::try_new(schema.clone(), vec![Arc::new(array1) as ArrayRef]).unwrap();

// In this batch, 'a' will have the same dictionary key as 'a' in the previous batch,
// and 'd' will take the next available key.
builder.append("a").unwrap();
builder.append("d").unwrap();
let array2 = builder.finish_preserve_values();
let batch2 = RecordBatch::try_new(schema.clone(), vec![Arc::new(array2) as ArrayRef]).unwrap();

let mut stream = vec![];
// You must set `.with_dictionary_handling(DictionaryHandling::Delta)` to
// enable delta dictionaries in the writer
let options = IpcWriteOptions::default().with_dictionary_handling(DictionaryHandling::Delta);
let mut writer = StreamWriter::try_new(&mut stream, &schema).unwrap();

// When writing the first batch, a dictionary message with 'a' and 'b' will be written
// prior to the record batch.
writer.write(&batch1).unwrap();
// With the second batch only a delta dictionary with 'd' will be written
// prior to the record batch. This is only possible with `finish_preserve_values`.
// Without it, 'a' and 'd' in this batch would have different keys than the
// first batch and so we'd have to send a replacement dictionary with new keys
// for both.
writer.write(&batch2).unwrap();
writer.finish().unwrap();
```

The object to write to

IPC write options

Whether the writer footer has been written, and the writer is finished

Keeps track of dictionaries that have been written

[Source](about:blank/src/arrow_ipc/writer.rs.html#1346-1353)
[§](#impl-StreamWriter%3CBufWriter%3CW%3E%3E)

[Source](about:blank/src/arrow_ipc/writer.rs.html#1355-1503)
[§](#impl-StreamWriter%3CW%3E)

[Source](about:blank/src/arrow_ipc/writer.rs.html#1363-1366)

Try to create a new writer, with the schema written as part of the header.

Note that there is no internal buffering. See also [`StreamWriter::try_new_buffered`](about:blank/struct.StreamWriter.html#method.try_new_buffered "associated function arrow_ipc::writer::StreamWriter::try_new_buffered").

##### [§](#errors)Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if writing the header to the writer fails.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1373-1396)

Try to create a new writer with [`IpcWriteOptions`](struct.IpcWriteOptions.html "struct arrow_ipc::writer::IpcWriteOptions").

##### [§](#errors-1)Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if writing the header to the writer fails.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1399-1422)

Write a record batch to the stream

[Source](about:blank/src/arrow_ipc/writer.rs.html#1425-1437)

Write continuation bytes, and mark the stream as done

[Source](about:blank/src/arrow_ipc/writer.rs.html#1440-1442)

Gets a reference to the underlying writer.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1447-1449)

Gets a mutable reference to the underlying writer.

It is inadvisable to directly write to the underlying writer.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1454-1457)

Flush the underlying writer.

Both the BufWriter and the underlying writer are flushed.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1496-1502)

Unwraps the the underlying writer.

The writer is flushed and the StreamWriter is finished before returning.

##### [§](#errors-2)Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if an error occurs while finishing the StreamWriter or while flushing the writer.

##### [§](#example)Example

```
// The result we expect from an empty schema
let expected = vec![
    255, 255, 255, 255,  48,   0,   0,   0,
     16,   0,   0,   0,   0,   0,  10,   0,
     12,   0,  10,   0,   9,   0,   4,   0,
     10,   0,   0,   0,  16,   0,   0,   0,
      0,   1,   4,   0,   8,   0,   8,   0,
      0,   0,   4,   0,   8,   0,   0,   0,
      4,   0,   0,   0,   0,   0,   0,   0,
    255, 255, 255, 255,   0,   0,   0,   0
];

let schema = Schema::empty();
let buffer: Vec<u8> = Vec::new();
let options = IpcWriteOptions::try_new(8, false, MetadataVersion::V5)?;
let stream_writer = StreamWriter::try_new_with_options(buffer, &schema, options)?;

assert_eq!(stream_writer.into_inner()?, expected);
```

[Source](about:blank/src/arrow_ipc/writer.rs.html#1505-1513)
[§](#impl-RecordBatchWriter-for-StreamWriter%3CW%3E)

[Source](about:blank/src/arrow_ipc/writer.rs.html#1506-1508)
[§](#method.write-1)

Write a single batch to the writer.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1510-1512)
[§](#method.close)

Write footer or termination data, then mark the writer as done.
