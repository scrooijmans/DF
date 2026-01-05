# Struct StringViewArrayBuilder Copy item path

<a href="https://docs.rs/datafusion-functions/50.2.0/x86_64-unknown-linux-gnu/src/datafusion_functions/strings.rs.html#130" class="src">Source</a>

``` rust
pub struct StringViewArrayBuilder { /* private fields */ }
```

## Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#impl-StringViewArrayBuilder" class="anchor">§</a>

### impl <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html" class="struct" title="struct datafusion::functions::strings::StringViewArrayBuilder">StringViewArrayBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#method.with_capacity" class="fn">with_capacity</a>( \_item_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, data_capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, ) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html" class="struct" title="struct datafusion::functions::strings::StringViewArrayBuilder">StringViewArrayBuilder</a>

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#method.write" class="fn">write</a>\<const CHECK_VALID: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>\>( &mut self, column: &<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/enum.ColumnarValueRef.html" class="enum" title="enum datafusion::functions::strings::ColumnarValueRef">ColumnarValueRef</a>\<'\_\>, i: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, )

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#method.append_offset" class="fn">append_offset</a>(&mut self)

#### pub fn <a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#method.finish" class="fn">finish</a>(self) -\> <a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/array/struct.GenericByteViewArray.html" class="struct" title="struct datafusion::common::arrow::array::GenericByteViewArray">GenericByteViewArray</a>\<<a href="https://docs.rs/datafusion/50.2.0/datafusion/common/arrow/datatypes/struct.StringViewType.html" class="struct" title="struct datafusion::common::arrow::datatypes::StringViewType">StringViewType</a>\>

## Auto Trait Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/datafusion/50.2.0/datafusion/functions/strings/struct.StringViewArrayBuilder.html#blanket-implementations" class="anchor">§</a>
