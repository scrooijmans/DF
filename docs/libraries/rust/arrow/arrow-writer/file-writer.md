# FileWriter in arrow_ipc::writer - Rust

```
pub struct FileWriter<W> {
    writer: W,
    write_options: IpcWriteOptions,
    schema: SchemaRef,
    block_offsets: usize,
    dictionary_blocks: Vec<Block>,
    record_blocks: Vec<Block>,
    finished: bool,
    dictionary_tracker: DictionaryTracker,
    custom_metadata: HashMap<String, String>,
    data_gen: IpcDataGenerator,
    compression_context: CompressionContext,
}
```

Expand description

Arrow File Writer

Writes Arrow \[`RecordBatch`\]es in the [IPC File Format](https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format).

## [§](#see-also)See Also

- [`StreamWriter`](struct.StreamWriter.html "struct arrow_ipc::writer::StreamWriter") for writing IPC Streams

## [§](#example)Example

```
let batch = record_batch!(("a", Int32, [1, 2, 3])).unwrap();
// create a new writer, the schema must be known in advance
let mut writer = FileWriter::try_new(&mut file, &batch.schema()).unwrap();
// write each batch to the underlying writer
writer.write(&batch).unwrap();
// When all batches are written, call finish to flush all buffers
writer.finish().unwrap();
```

The object to write to

IPC write options

A reference to the schema, used in validating record batches

The number of bytes between each block of bytes, as an offset for random access

Dictionary blocks that will be written as part of the IPC footer

Record blocks that will be written as part of the IPC footer

Whether the writer footer has been written, and the writer is finished

Keeps track of dictionaries that have been written

User level customized metadata

[Source](about:blank/src/arrow_ipc/writer.rs.html#1057-1064)
[§](#impl-FileWriter%3CBufWriter%3CW%3E%3E)

[Source](about:blank/src/arrow_ipc/writer.rs.html#1061-1063)

Try to create a new file writer with the writer wrapped in a BufWriter.

See [`FileWriter::try_new`](about:blank/struct.FileWriter.html#method.try_new "associated function arrow_ipc::writer::FileWriter::try_new") for an unbuffered version.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1066-1246)
[§](#impl-FileWriter%3CW%3E)

[Source](about:blank/src/arrow_ipc/writer.rs.html#1074-1077)

Try to create a new writer, with the schema written as part of the header

Note the created writer is not buffered. See [`FileWriter::try_new_buffered`](about:blank/struct.FileWriter.html#method.try_new_buffered "associated function arrow_ipc::writer::FileWriter::try_new_buffered") for details.

##### [§](#errors)Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if writing the header to the writer fails.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1086-1118)

Try to create a new writer with IpcWriteOptions

Note the created writer is not buffered. See [`FileWriter::try_new_buffered`](about:blank/struct.FileWriter.html#method.try_new_buffered "associated function arrow_ipc::writer::FileWriter::try_new_buffered") for details.

##### [§](#errors-1)Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if writing the header to the writer fails.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1121-1123)

Adds a key-value pair to the [FileWriter](struct.FileWriter.html "struct arrow_ipc::writer::FileWriter")’s custom metadata

[Source](about:blank/src/arrow_ipc/writer.rs.html#1126-1160)

Write a record batch to the file

[Source](about:blank/src/arrow_ipc/writer.rs.html#1163-1204)

Write footer and closing tag, then mark the writer as done

[Source](about:blank/src/arrow_ipc/writer.rs.html#1207-1209)

Returns the arrow \[`SchemaRef`\] for this arrow file.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1212-1214)

Gets a reference to the underlying writer.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1219-1221)

Gets a mutable reference to the underlying writer.

It is inadvisable to directly write to the underlying writer.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1226-1229)

Flush the underlying writer.

Both the BufWriter and the underlying writer are flushed.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1239-1245)

Unwraps the underlying writer.

The writer is flushed and the FileWriter is finished before returning.

##### [§](#errors-2)Errors

An [‘Err’](https://doc.rust-lang.org/nightly/core/result/enum.Result.html#variant.Err "variant core::result::Result::Err") may be returned if an error occurs while finishing the StreamWriter or while flushing the writer.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1248-1256)
[§](#impl-RecordBatchWriter-for-FileWriter%3CW%3E)

[Source](about:blank/src/arrow_ipc/writer.rs.html#1249-1251)
[§](#method.write-1)

Write a single batch to the writer.

[Source](about:blank/src/arrow_ipc/writer.rs.html#1253-1255)
[§](#method.close)

Write footer or termination data, then mark the writer as done.
