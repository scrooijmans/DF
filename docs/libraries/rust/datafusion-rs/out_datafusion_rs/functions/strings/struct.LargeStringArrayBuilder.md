# Struct LargeStringArrayBuilder Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/strings.rs.html#199" class="src">Source</a>

``` rust
pub struct LargeStringArrayBuilder { /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#impl-LargeStringArrayBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html" class="struct" title="struct datafusion::functions::strings::LargeStringArrayBuilder">LargeStringArrayBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#method.with_capacity" class="fn">with_capacity</a>( item_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, data_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html" class="struct" title="struct datafusion::functions::strings::LargeStringArrayBuilder">LargeStringArrayBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#method.write" class="fn">write</a>\<const CHECK_VALID: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>( &mut self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/enum.ColumnarValueRef.html" class="enum" title="enum datafusion::functions::strings::ColumnarValueRef">ColumnarValueRef</a>\<'\_\>, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#method.append_offset" class="fn">append_offset</a>(&mut self)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#method.finish" class="fn">finish</a>( self, null_buffer: <a href="https://doc.rust-lang.org/nightly/core/option/enum.Option.html" class="enum" title="enum core::option::Option">Option</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/buffer/struct.NullBuffer.html" class="struct" title="struct datafusion::common::arrow::buffer::NullBuffer">NullBuffer</a>\>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteArray">GenericByteArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.GenericStringType.html" class="struct" title="struct datafusion::common::arrow::datatypes::GenericStringType">GenericStringType</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.i64.html" class="primitive">i64</a>\>\>

Finalize the builder into a concrete [`LargeStringArray`](https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/type.LargeStringArray.html "type datafusion::common::arrow::array::LargeStringArray").

##### <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#panics" class="doc-anchor">§</a>Panics

This method can panic when:

- the provided `null_buffer` is not the same length as the `offsets_buffer`.

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.LargeStringArrayBuilder.html#blanket-implementations" class="anchor">§</a>
