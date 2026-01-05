# Module ffi_stream Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/lib.rs.html#255" class="src">Source</a>

Available on **crate feature `ffi`** only.

Expand description

Contains declarations to bind to the [C Stream Interface](https://arrow.apache.org/docs/format/CStreamInterface.html).

This module has two main interfaces: One interface maps C ABI to native Rust types, i.e. convert c-pointers, c_char, to native rust. This is handled by [FFI_ArrowArrayStream](https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html "struct arrow::ffi_stream::FFI_ArrowArrayStream").

The second interface is used to import `FFI_ArrowArrayStream` as Rust implementation `RecordBatch` reader. This is handled by `ArrowArrayStreamReader`.

<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/index.html#" class="tooltip" title="This example is not tested">ⓘ</a>

``` rust
// create an record batch reader natively
let file = File::open("arrow_file").unwrap();
let reader = Box::new(FileReader::try_new(file).unwrap());

// export it
let mut stream = FFI_ArrowArrayStream::empty();
unsafe { export_reader_into_raw(reader, &mut stream) };

// consumed and used by something else...

// import it
let stream_reader = unsafe { ArrowArrayStreamReader::from_raw(&mut stream).unwrap() };
let imported_schema = stream_reader.schema();

let mut produced_batches = vec![];
for batch in stream_reader {
     produced_batches.push(batch.unwrap());
}
Ok(())
}
```

## Structs<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/index.html#structs" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.ArrowArrayStreamReader.html" class="struct" title="struct arrow::ffi_stream::ArrowArrayStreamReader">ArrowArrayStreamReader</a>  
A `RecordBatchReader` which imports Arrays from `FFI_ArrowArrayStream`.

<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>  
ABI-compatible struct for `ArrayStream` from C Stream Interface See <https://arrow.apache.org/docs/format/CStreamInterface.html#structure-definitions> This was created by bindgen
