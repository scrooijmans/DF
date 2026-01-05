# Struct FFI_ArrowArray Copy item path

<a href="https://docs.rs/arrow-data/56.2.0/x86_64-unknown-linux-gnu/src/arrow_data/ffi.rs.html#39" class="src">Source</a>

``` rust
#[repr(C)]pub struct FFI_ArrowArray { /* private fields */ }
```

Available on **crate feature `ffi`** only.

Expand description

ABI-compatible struct for ArrowArray from C Data Interface See <https://arrow.apache.org/docs/format/CDataInterface.html#structure-definitions>

``` rust
fn export_array(array: &ArrayData) -> FFI_ArrowArray {
    FFI_ArrowArray::new(array)
}
```

## Implementations<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#impl-FFI_ArrowArray" class="anchor">§</a>

### impl <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.new" class="fn">new</a>(data: &<a href="https://docs.rs/arrow/latest/arrow/array/struct.ArrayData.html" class="struct" title="struct arrow::array::ArrayData">ArrayData</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

creates a new `FFI_ArrowArray` from existing data.

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.from_raw" class="fn">from_raw</a>(array: <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*mut</a> <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>) -\> <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

Takes ownership of the pointed to [`FFI_ArrowArray`](https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html "struct arrow::ffi::FFI_ArrowArray")

This acts to [move](https://arrow.apache.org/docs/format/CDataInterface.html#moving-an-array) the data out of `array`, setting the release callback to NULL

##### <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#safety" class="doc-anchor">§</a>Safety

- `array` must be [valid](https://doc.rust-lang.org/std/ptr/index.html#safety) for reads and writes
- `array` must be properly aligned
- `array` must point to a properly initialized value of [`FFI_ArrowArray`](https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html "struct arrow::ffi::FFI_ArrowArray")

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.empty" class="fn">empty</a>() -\> <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

create an empty `FFI_ArrowArray`, which can be used to import data into

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.len" class="fn">len</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

the length of the array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.is_empty" class="fn">is_empty</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

whether the array is empty

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.is_released" class="fn">is_released</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>

Whether the array has been released

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.offset" class="fn">offset</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

the offset of the array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.null_count" class="fn">null_count</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

the null count of the array

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.null_count_opt" class="fn">null_count_opt</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>\>

Returns the null count, checking for validity

#### pub unsafe fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.set_null_count" class="fn">set_null_count</a>(&mut self, null_count: <a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>)

Set the null count of the array

##### <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#safety-1" class="doc-anchor">§</a>Safety

Null count must match that of null buffer

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.buffer" class="fn">buffer</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.pointer.html" class="primitive">*const</a> <a href="https://doc.rust-lang.org/nightly/std/primitive.u8.html" class="primitive">u8</a>

Returns the buffer at the provided index

##### <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#panic" class="doc-anchor">§</a>Panic

Panics if index \>= self.num_buffers() or the buffer is not correctly aligned

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.num_buffers" class="fn">num_buffers</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of buffers

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.child" class="fn">child</a>(&self, index: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> &<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

Returns the child at the provided index

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.num_children" class="fn">num_children</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>

Returns the number of children

#### pub fn <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.dictionary" class="fn">dictionary</a>(&self) -\> <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<&<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>\>

Returns the dictionary if any

## Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#impl-Debug-for-FFI_ArrowArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html" class="trait" title="trait core::fmt::Debug">Debug</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.fmt" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt" class="fn">fmt</a>(&self, f: &mut <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html" class="struct" title="struct core::fmt::Formatter">Formatter</a>\<'\_\>) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.unit.html" class="primitive">()</a>, <a href="https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html" class="struct" title="struct core::fmt::Error">Error</a>\>

Formats the value using the given formatter. [Read more](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt)

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#impl-Drop-for-FFI_ArrowArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html" class="trait" title="trait core::ops::drop::Drop">Drop</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#method.drop" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop" class="fn">drop</a>(&mut self)

Executes the destructor for this type. [Read more](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop)

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#impl-Send-for-FFI_ArrowArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Send.html" class="trait" title="trait core::marker::Send">Send</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#impl-Sync-for-FFI_ArrowArray" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html" class="trait" title="trait core::marker::Sync">Sync</a> for <a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html" class="struct" title="struct arrow::ffi::FFI_ArrowArray">FFI_ArrowArray</a>

## Auto Trait Implementations<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/arrow/latest/arrow/ffi/struct.FFI_ArrowArray.html#blanket-implementations" class="anchor">§</a>
