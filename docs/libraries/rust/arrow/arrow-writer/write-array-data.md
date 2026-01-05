# write_array_data in arrow_ipc::writer - Rust

[arrow_ipc](../index.html)::[writer](index.html)

## Function write_array_data 

[Source](about:blank/src/arrow_ipc/writer.rs.html#1715-1965)

```
fn write_array_data(
    array_data: &ArrayData,
    buffers: &mut Vec<Buffer>,
    arrow_data: &mut Vec<u8>,
    nodes: &mut Vec<FieldNode>,
    offset: i64,
    num_rows: usize,
    null_count: usize,
    compression_codec: Option<CompressionCodec>,
    compression_context: &mut CompressionContext,
    write_options: &IpcWriteOptions,
) -> Result<i64, ArrowError>
```

Expand description

Write array data to a vector of bytes
