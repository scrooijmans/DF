# Struct FFI_ArrowArrayStream Copy item path

<a href="https://docs.rs/arrow-array/56.2.0/x86_64-unknown-linux-gnu/src/arrow_array/ffi_stream.rs.html#87" class="src">Source</a>

``` rust
#[repr(C)]pub struct FFI_ArrowArrayStream {
    pub get_schema: Option<unsafe extern "C" fn(*mut FFI_ArrowArrayStream, *mut FFI_ArrowSchema) -> i32>,
    pub get_next: Option<unsafe extern "C" fn(*mut FFI_ArrowArrayStream, *mut FFI_ArrowArray) -> i32>,
    pub get_last_error: Option<unsafe extern "C" fn(*mut FFI_ArrowArrayStream) -> *const i8>,
    pub release: Option<unsafe extern "C" fn(*mut FFI_ArrowArrayStream)>,
    pub private_data: *mut c_void,
}
```

Expand description

ABI-compatible struct for `ArrayStream` from C Stream Interface See <https://arrow.apache.org/docs/format/CStreamInterface.html#structure-definitions> This was created by bindgen

## Fields<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#fields" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#structfield.get_schema" class="anchor field">§</a>`get_schema: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<unsafe extern "C" `<a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive"><code>fn</code></a>`(`<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*mut</code></a>` `<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream"><code>FFI_ArrowArrayStream</code></a>`, `<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*mut</code></a>` `<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowSchema.html" class="struct" title="struct arrow::ffi::FFI_ArrowSchema"><code>FFI_ArrowSchema</code></a>`) -> `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>`

C function to get schema from the stream

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#structfield.get_next" class="anchor field">§</a>`get_next: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<unsafe extern "C" `<a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive"><code>fn</code></a>`(`<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*mut</code></a>` `<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream"><code>FFI_ArrowArrayStream</code></a>`, `<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*mut</code></a>` `<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray"><code>FFI_ArrowArray</code></a>`) -> `<a href="https://doc.rust-lang.org/nightly/std/primitive.i32.html" class="primitive"><code>i32</code></a>`>`

C function to get next array from the stream

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#structfield.get_last_error" class="anchor field">§</a>`get_last_error: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<unsafe extern "C" `<a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive"><code>fn</code></a>`(`<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*mut</code></a>` `<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream"><code>FFI_ArrowArrayStream</code></a>`) -> `<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*const</code></a>` `<a href="https://doc.rust-lang.org/nightly/std/primitive.i8.html" class="primitive"><code>i8</code></a>`>`

C function to get the error from last operation on the stream

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#structfield.release" class="anchor field">§</a>`release: `<a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option"><code>Option</code></a>`<unsafe extern "C" `<a href="https://doc.rust-lang.org/nightly/std/primitive.fn.html" class="primitive"><code>fn</code></a>`(`<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*mut</code></a>` `<a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream"><code>FFI_ArrowArrayStream</code></a>`)>`

C function to release the stream

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#structfield.private_data" class="anchor field">§</a>`private_data: `<a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive"><code>*mut</code></a>` `<a href="https://doc.rust-lang.org/nightly/core/ffi/enum.c_void.html" class="enum" title="enum core::ffi::c_void"><code>c_void</code></a>

Private data used by the stream

## Implementations<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#impl-FFI_ArrowArrayStream" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#method.new" class="fn">new</a>( batch_reader: <a href="https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html" class="struct" title="struct alloc::boxed::Box">Box</a>\<dyn <a href="https://docs.rs/arrow/latest/arrow/array/trait.RecordBatchReader.html" class="trait" title="trait arrow::array::RecordBatchReader">RecordBatchReader</a>\<Item = <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/arrow/latest/arrow/array/struct.RecordBatch.html" class="struct" title="struct arrow::array::RecordBatch">RecordBatch</a>, <a href="https://docs.rs/arrow/latest/arrow/error/enum.ArrowError.html" class="enum" title="enum arrow::error::ArrowError">ArrowError</a>\>\> + <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a>\>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>

Creates a new [`FFI_ArrowArrayStream`](https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html "struct arrow::ffi_stream::FFI_ArrowArrayStream").

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#method.from_raw" class="fn">from_raw</a>( raw_stream: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*mut</a> <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>, ) -\> <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>

Takes ownership of the pointed to [`FFI_ArrowArrayStream`](https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html "struct arrow::ffi_stream::FFI_ArrowArrayStream")

This acts to [move](https://arrow.apache.org/docs/format/CDataInterface.html#moving-an-array) the data out of `raw_stream`, setting the release callback to NULL

##### <a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#safety" class="doc-anchor">§</a>Safety

- `raw_stream` must be [valid](https://doc.rust-lang.org/std/ptr/index.html#safety) for reads and writes
- `raw_stream` must be properly aligned
- `raw_stream` must point to a properly initialized value of [`FFI_ArrowArrayStream`](https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html "struct arrow::ffi_stream::FFI_ArrowArrayStream")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>

Creates a new empty [FFI_ArrowArrayStream](https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html "struct arrow::ffi_stream::FFI_ArrowArrayStream"). Used to import from the C Stream Interface.

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#impl-Debug-for-FFI_ArrowArrayStream" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#impl-Drop-for-FFI_ArrowArrayStream" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#impl-Send-for-FFI_ArrowArrayStream" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi_stream/struct.FFI_ArrowArrayStream.html" class="struct" title="struct arrow::ffi_stream::FFI_ArrowArrayStream">FFI_ArrowArrayStream</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/array/ffi_stream/struct.FFI_ArrowArrayStream.html#blanket-implementations" class="anchor">§</a>
