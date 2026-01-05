# Enum AnyValueBufferTrusted Copy item path

<a href="https://docs.rs/polars-core/0.51.0/x86_64-unknown-linux-gnu/src/polars_core/frame/row/av_buffer.rs.html#329" class="src">Source</a>

``` rust
pub enum AnyValueBufferTrusted<'a> {
Show 15 variants    Boolean(BooleanChunkedBuilder),
    Int8(PrimitiveChunkedBuilder<Int8Type>),
    Int16(PrimitiveChunkedBuilder<Int16Type>),
    Int32(PrimitiveChunkedBuilder<Int32Type>),
    Int64(PrimitiveChunkedBuilder<Int64Type>),
    UInt8(PrimitiveChunkedBuilder<UInt8Type>),
    UInt16(PrimitiveChunkedBuilder<UInt16Type>),
    UInt32(PrimitiveChunkedBuilder<UInt32Type>),
    UInt64(PrimitiveChunkedBuilder<UInt64Type>),
    Float32(PrimitiveChunkedBuilder<Float32Type>),
    Float64(PrimitiveChunkedBuilder<Float64Type>),
    String(BinViewChunkedBuilder<str>),
    Struct(BitmapBuilder, Vec<(AnyValueBuffer<'a>, PlSmallStr)>),
    Null(NullChunkedBuilder),
    All(DataType, Vec<AnyValue<'a>>),
}
```

Expand description

An [`AnyValueBuffer`](https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBuffer.html "enum polars::frame::row::AnyValueBuffer") that should be used when we trust the builder

## Variants<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variants" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Boolean" class="anchor">§</a>

### Boolean(<a href="https://docs.rs/polars/latest/polars/prelude/struct.BooleanChunkedBuilder.html" class="struct" title="struct polars::prelude::BooleanChunkedBuilder">BooleanChunkedBuilder</a>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Int8" class="anchor">§</a>

### Int8(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int8Type.html" class="struct" title="struct polars::prelude::Int8Type">Int8Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Int16" class="anchor">§</a>

### Int16(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int16Type.html" class="struct" title="struct polars::prelude::Int16Type">Int16Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Int32" class="anchor">§</a>

### Int32(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int32Type.html" class="struct" title="struct polars::prelude::Int32Type">Int32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Int64" class="anchor">§</a>

### Int64(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Int64Type.html" class="struct" title="struct polars::prelude::Int64Type">Int64Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.UInt8" class="anchor">§</a>

### UInt8(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt8Type.html" class="struct" title="struct polars::prelude::UInt8Type">UInt8Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.UInt16" class="anchor">§</a>

### UInt16(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt16Type.html" class="struct" title="struct polars::prelude::UInt16Type">UInt16Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.UInt32" class="anchor">§</a>

### UInt32(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt32Type.html" class="struct" title="struct polars::prelude::UInt32Type">UInt32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.UInt64" class="anchor">§</a>

### UInt64(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.UInt64Type.html" class="struct" title="struct polars::prelude::UInt64Type">UInt64Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Float32" class="anchor">§</a>

### Float32(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float32Type.html" class="struct" title="struct polars::prelude::Float32Type">Float32Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Float64" class="anchor">§</a>

### Float64(<a href="https://docs.rs/polars/latest/polars/prelude/struct.PrimitiveChunkedBuilder.html" class="struct" title="struct polars::prelude::PrimitiveChunkedBuilder">PrimitiveChunkedBuilder</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Float64Type.html" class="struct" title="struct polars::prelude::Float64Type">Float64Type</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.String" class="anchor">§</a>

### String(<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.BinViewChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::BinViewChunkedBuilder">BinViewChunkedBuilder</a>\<<a href="https://doc.rust-lang.org/nightly/std/primitive.str.html" class="primitive">str</a>\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Struct" class="anchor">§</a>

### Struct(<a href="https://docs.rs/polars-arrow/0.51.0/x86_64-unknown-linux-gnu/polars_arrow/bitmap/builder/struct.BitmapBuilder.html" class="struct" title="struct polars_arrow::bitmap::builder::BitmapBuilder">BitmapBuilder</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<(<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBuffer.html" class="enum" title="enum polars::frame::row::AnyValueBuffer">AnyValueBuffer</a>\<'a\>, <a href="https://docs.rs/polars/latest/polars/prelude/struct.PlSmallStr.html" class="struct" title="struct polars::prelude::PlSmallStr">PlSmallStr</a>)\>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.Null" class="anchor">§</a>

### Null(<a href="https://docs.rs/polars/latest/polars/chunked_array/builder/struct.NullChunkedBuilder.html" class="struct" title="struct polars::chunked_array::builder::NullChunkedBuilder">NullChunkedBuilder</a>)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#variant.All" class="anchor">§</a>

### All(<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html" class="struct" title="struct alloc::vec::Vec">Vec</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'a\>\>)

## Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#impl-AnyValueBufferTrusted%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html" class="enum" title="enum polars::frame::row::AnyValueBufferTrusted">AnyValueBufferTrusted</a>\<'a\>

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.new" class="fn">new</a>(dtype: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, len: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>) -\> <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html" class="enum" title="enum polars::frame::row::AnyValueBufferTrusted">AnyValueBufferTrusted</a>\<'a\>

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.add_unchecked_owned_physical" class="fn">add_unchecked_owned_physical</a>(&mut self, val: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>)

Will add the [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") into [`Self`](https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html "enum polars::frame::row::AnyValueBufferTrusted") and unpack as the physical type belonging to [`Self`](https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html "enum polars::frame::row::AnyValueBufferTrusted"). This should only be used with physical buffers

If a type is not primitive or String, the AnyValues will be converted to static

##### <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#safety" class="doc-anchor">§</a>Safety

The caller must ensure that the [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") type exactly matches the `Buffer` type and is owned.

#### pub unsafe fn <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.add_unchecked_borrowed_physical" class="fn">add_unchecked_borrowed_physical</a>(&mut self, val: &<a href="https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html" class="enum" title="enum polars::prelude::AnyValue">AnyValue</a>\<'\_\>)

##### <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#safety-1" class="doc-anchor">§</a>Safety

The caller must ensure that the [`AnyValue`](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html "enum polars::prelude::AnyValue") type exactly matches the `Buffer` type and is borrowed.

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.reset" class="fn">reset</a>( &mut self, capacity: <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>, strict: <a href="https://doc.rust-lang.org/nightly/std/primitive.bool.html" class="primitive">bool</a>, ) -\> <a href="https://doc.rust-lang.org/nightly/core/result/enum.Result.html" class="enum" title="enum core::result::Result">Result</a>\<<a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>, <a href="https://docs.rs/polars/latest/polars/prelude/enum.PolarsError.html" class="enum" title="enum polars::prelude::PolarsError">PolarsError</a>\>

Clear `self` and give `capacity`, returning the old contents as a [`Series`](https://docs.rs/polars/latest/polars/prelude/struct.Series.html "struct polars::prelude::Series").

#### pub fn <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.into_series" class="fn">into_series</a>(self) -\> <a href="https://docs.rs/polars/latest/polars/prelude/struct.Series.html" class="struct" title="struct polars::prelude::Series">Series</a>

## Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#trait-implementations" class="anchor">§</a>

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#impl-Clone-for-AnyValueBufferTrusted%3C&#39;a%3E" class="anchor">§</a>

### impl\<'a\> <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html" class="trait" title="trait core::clone::Clone">Clone</a> for <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html" class="enum" title="enum polars::frame::row::AnyValueBufferTrusted">AnyValueBufferTrusted</a>\<'a\>

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.clone" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone" class="fn">clone</a>(&self) -\> <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html" class="enum" title="enum polars::frame::row::AnyValueBufferTrusted">AnyValueBufferTrusted</a>\<'a\>

Returns a duplicate of the value. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#tymethod.clone)

1.0.0 · <a href="https://doc.rust-lang.org/nightly/src/core/clone.rs.html#245-247" class="src">Source</a><a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.clone_from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from" class="fn">clone_from</a>(&mut self, source: &Self)

Performs copy-assignment from `source`. [Read more](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#impl-From%3C(%26DataType,+usize)%3E-for-AnyValueBufferTrusted%3C&#39;_%3E" class="anchor">§</a>

### impl <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html" class="trait" title="trait core::convert::From">From</a>\<(&<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)\> for <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html" class="enum" title="enum polars::frame::row::AnyValueBufferTrusted">AnyValueBufferTrusted</a>\<'\_\>

<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#method.from" class="anchor">§</a>

#### fn <a href="https://doc.rust-lang.org/nightly/core/convert/trait.From.html#tymethod.from" class="fn">from</a>(a: (&<a href="https://docs.rs/polars/latest/polars/prelude/enum.DataType.html" class="enum" title="enum polars::prelude::DataType">DataType</a>, <a href="https://doc.rust-lang.org/nightly/std/primitive.usize.html" class="primitive">usize</a>)) -\> <a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html" class="enum" title="enum polars::frame::row::AnyValueBufferTrusted">AnyValueBufferTrusted</a>\<'\_\>

Converts to this type from the input type.

## Auto Trait Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#synthetic-implementations" class="anchor">§</a>

## Blanket Implementations<a href="https://docs.rs/polars/latest/polars/frame/row/enum.AnyValueBufferTrusted.html#blanket-implementations" class="anchor">§</a>
