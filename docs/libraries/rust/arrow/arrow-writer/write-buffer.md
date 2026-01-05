# write_buffer in arrow_ipc::writer - Rust

## Function write_buffer 

[Source](about:blank/src/arrow_ipc/writer.rs.html#1979-2007)

```
fn write_buffer(
    buffer: &[u8],
    buffers: &mut Vec<Buffer>,
    arrow_data: &mut Vec<u8>,
    offset: i64,
    compression_codec: Option<CompressionCodec>,
    compression_context: &mut CompressionContext,
    alignment: u8,
) -> Result<i64, ArrowError>
```

Expand description

Write a buffer into `arrow_data`, a vector of bytes, and adds its [`crate::Buffer`](../gen/Schema/struct.Buffer.html "struct arrow_ipc::gen::Schema::Buffer") to `buffers`. Returns the new offset in `arrow_data`

From [https://github.com/apache/arrow/blob/6a936c4ff5007045e86f65f1a6b6c3c955ad5103/format/Message.fbs#L58](https://github.com/apache/arrow/blob/6a936c4ff5007045e86f65f1a6b6c3c955ad5103/format/Message.fbs#L58) Each constituent buffer is first compressed with the indicated compressor, and then written with the uncompressed length in the first 8 bytes as a 64-bit little-endian signed integer followed by the compressed buffer bytes (and then padding as required by the protocol). The uncompressed length may be set to -1 to indicate that the data that follows is not compressed, which can be useful for cases where compression does not yield appreciable savings.
