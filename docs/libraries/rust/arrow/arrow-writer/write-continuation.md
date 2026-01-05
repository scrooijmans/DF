# write_continuation in arrow_ipc::writer - Rust

[arrow_ipc](../index.html)::[writer](index.html)

## Function write_continuation 

[Source](about:blank/src/arrow_ipc/writer.rs.html#1590-1621)

```
fn write_continuation<W: Write>(
    writer: W,
    write_options: &IpcWriteOptions,
    total_len: i32,
) -> Result<usize, ArrowError>
```

Expand description

Write a record batch to the writer, writing the message size before the message if the record batch is being written to a stream
